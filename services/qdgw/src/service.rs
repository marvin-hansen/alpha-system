use std::error::Error;
use std::future::Future;

use fluvio::{Offset, PartitionConsumer};
use futures::StreamExt;
use tokio::{pin, select};

use crate::handle;

pub struct Server {
    consumer: PartitionConsumer,
}

impl Server {
    pub fn new(consumer: PartitionConsumer) -> Self {
        Self { consumer }
    }
}

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), Box<dyn Error + Send>> {
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
                                        handle::handle_record(&record).await;
                                },
                                    Err(err) =>{
                                        return Err(Box::new(err));
                                    }
                            }
                        },
                        None => {},
                    }

                }// end stream.next()
            } // end select
        } // end loop

        return Ok(());
    }
}
