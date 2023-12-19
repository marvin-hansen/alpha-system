use crate::service::Server;
use common::prelude::MessageProcessingError;
use fluvio::dataplane::record::ConsumerRecord;
use sbe_messages::prelude::{MessageType, StartDataMessage, StopAllDataMessage, StopDataMessage};

impl Server {
    /// Handles an incoming record from the Fluvio stream.
    ///
    /// # Parameters
    /// * `record`: The incoming Fluvio consumer record to handle.
    ///
    /// # Functionality
    /// - Extracts the message value from the record and converts it to a byte buffer.
    /// - Deserializes the message type from the buffer.
    /// - Matches on the message type:
    ///   - `NullVal`: Logs receiving a null value message.
    ///   - `StartData`: Deserializes a `StartDataMessage` and calls `start_date`.
    ///   - `StopData`: Deserializes a `StopDataMessage` and calls `stop_date`.
    ///   - `StopAllData`: Deserializes a `StopAllDataMessage` and calls `stop_all_data`.
    pub(crate) async fn handle_record(
        &self,
        record: &ConsumerRecord,
    ) -> Result<(), MessageProcessingError> {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2]);

        match message_type {
            MessageType::NullVal => Err(MessageProcessingError(
                "[QDGW/handle::handle_record]:  Fluvio consumer record contained a null value"
                    .to_string(),
            )),
            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(buffer);
                self.start_date(&start_data_msg).await
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


impl Server {

    async fn start_date(
        &self,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        println!(
            "[QDGW/handle::start_date]: start_data: {:?}",
            start_data_msg
        );

        Ok(())
    }

    async fn stop_date(
        &self,
        stop_data_msg: &StopDataMessage,
    ) -> Result<(), MessageProcessingError> {
        println!("[QDGW/handle::stop_date]: stop_data: {:?}", stop_data_msg);

        Ok(())
    }

    async fn stop_all_data(
        &self,
        stop_all_data_msg: &StopAllDataMessage,
    ) -> Result<(), MessageProcessingError> {
        println!(
            "[QDGW/handle::stop_all_data]: stop_all_data: {:?}",
            stop_all_data_msg
        );

        Ok(())
    }
}