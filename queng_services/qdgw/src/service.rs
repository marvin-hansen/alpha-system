use client_manager::ClientManager;
use common::errors::MessageProcessingError;
use fluvio::dataplane::record::ConsumerRecord;
use fluvio::{Offset, PartitionConsumer};
use futures::StreamExt;
use qd_manager::QDManager;
use sbe_messages::prelude::{
    ClientLoginMessage, ClientLogoutMessage, MessageType, StartDataMessage, StopAllDataMessage,
    StopDataMessage,
};
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::{pin, select};

pub struct Server {
    consumer: PartitionConsumer,
    qd_manager: QDManager,
    client_manager: Arc<Mutex<ClientManager>>,
}

impl Server {
    pub fn new(
        consumer: PartitionConsumer,
        qd_manager: QDManager,
        client_manager: Arc<Mutex<ClientManager>>,
    ) -> Self {
        Self {
            consumer,
            qd_manager,
            client_manager,
        }
    }
}

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output=()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        // Creates a stream of messages from the topic.
        let mut stream = self
            .consumer
            .stream(Offset::end())
            .await
            .expect("[QDGW/Service:run]: Failed to create a stream");

        loop {
            select! {
                    _ = &mut signal_future => {
                         break;
                    }

                    record = stream.next() => {
                        match record {
                            Some(res) => {
                                match res {
                                    Ok(record) => {
                                        match self.handle_record(&record).await{
                                        Ok(()) => {},
                                        Err(e) => {
                                            return Err(e);
                                        }
                                    }
                                },
                                    Err(e) =>{
                                        return Err(MessageProcessingError(e.to_string()));
                                    }
                            }
                        },
                        None => {}, // No message, no processing.
                    }

                }// end stream.next()
            } // end select
        } // end loop

        return Ok(());
    }

    /// Handles an incoming record from the Fluvio stream.
    ///
    /// # Parameters
    /// * `record`: The incoming Fluvio consumer record to handle.
    ///
    /// # Functionality
    /// - Extracts the message value from the record and converts it to a byte buffer.
    /// - Deserializes the message type from the buffer.
    /// - Matches on the message type:
    ///   - `UnknownMessageType`: Logs receiving an UnknownMessageType.
    ///   - `StartData`: Deserializes a `StartDataMessage` and calls `start_date`.
    ///   - `StopData`: Deserializes a `StopDataMessage` and calls `stop_date`.
    ///   - `StopAllData`: Deserializes a `StopAllDataMessage` and calls `stop_all_data`.
    async fn handle_record(&self, record: &ConsumerRecord) -> Result<(), MessageProcessingError> {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2]);

        match message_type {
            MessageType::UnknownMessageType => Err(MessageProcessingError(
                "[QDGW/handle::handle_record]:  Fluvio consumer record contained an unknown message type."
                    .to_string(),
            )),

            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(buffer);
                self.client_login(&self.client_manager, &client_login_msg).await
            }

            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(buffer);
                self.client_logout(&self.client_manager, &client_logout_msg).await
            }

            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(buffer);
                self.start_date(&self.client_manager, &self.qd_manager, &start_data_msg).await
            }
            MessageType::StopData => {
                let stop_data_msg = StopDataMessage::from(buffer);
                self.stop_date(&stop_data_msg).await
            }
            MessageType::StopAllData => {
                let stop_all_data_msg = StopAllDataMessage::from(buffer);
                self.stop_all_data(&stop_all_data_msg).await
            }
        }
    }
}
