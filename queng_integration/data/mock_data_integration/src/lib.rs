use std::fmt::Error;
use trait_data_integration::{ImsDataIntegration, StreamProcessor};

pub struct MockDataIntegration;

impl ImsDataIntegration for MockDataIntegration {
    async fn start_trade_date<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: StreamProcessor + Send + Sync + 'static,
    {
        println!("start_trade_date for symbols: {:?}", symbols);

        // generate 100 strings, send them to the processor,
        // and async wait for 5 ms before sending the next one.
        for _ in 0..100 {
            match processor.process(&[Vec::from("test".to_string())]).await {
                Ok(_) => {}
                Err(e) => println!("Error processing data: {}", e),
            };
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }

        Ok(())
    }

    async fn stop_trade_date(&self, symbols: &[String]) -> Result<(), Error> {
        println!("stop_trade_date for symbols: {:?}", symbols);

        Ok(())
    }

    async fn stop_all_trade_date(&self) -> Result<(), Error> {
        println!("stop_all_trade_date");

        Ok(())
    }
}
