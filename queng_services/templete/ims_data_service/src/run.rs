use crate::service::Server;
use common_errors::MessageProcessingError;
use std::error::Error;
use std::future::Future;
use tokio::{pin, select};

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, then pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        loop {
            select! {
                    _ = &mut signal_future => {
                    break;
                }

                // Else, poll and process messages

            } // end select
        } // end loop

        self.stop().await.expect("Failed to stop message service");

        Ok(())
    }
}

impl Server {
    pub(super) async fn stop(&self) -> Result<(), Box<dyn Error>> {
        // Delete stream and topic of the consumer before shutting down.

        // Logout user. Call it just once as consumer and producer use the same user.

        // Shutdown consumer

        // Shutdown producer

        Ok(())
    }
}
