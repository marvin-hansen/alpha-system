use common::prelude::MessageProcessingError;
use std::future::Future;
use tokio::{pin, select};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        loop {
            select! {
                   _ = &mut signal_future => {
                        break;
                   }
            } // end select
        } // end loop

        Ok(())
    }
}
