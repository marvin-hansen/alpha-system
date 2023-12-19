use common::errors::MessageProcessingError;
use fluvio::{Offset, PartitionConsumer};
use futures::StreamExt;
use qd_manager::QDManager;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::{pin, select};
use client_manager::ClientManager;

pub struct Server {
    consumer: PartitionConsumer,
    qd_manager: QDManager,
    client_manager: Arc<Mutex<ClientManager>>,

}

impl Server {
    pub fn new(
        consumer: PartitionConsumer,
        qd_manager: QDManager,
        client_manager: Arc<Mutex<ClientManager>>
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
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, pin the future.
        // https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
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
}
