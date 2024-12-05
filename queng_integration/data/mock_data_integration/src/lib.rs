use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

#[derive(Debug, Clone, Copy)]
pub struct ImsMockDataIntegration;

impl ImsMockDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImsDataIntegration for ImsMockDataIntegration {
    async fn start_date<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        println!("MockDataIntegration start_date");
        println!("Start data for symbols: {:#?}", symbols);
        // Iterate a 100 times to simulate data ingestion;
        // for each iteration call the processor to process the data
        // Then wait 5 ms async to simulate some processing
        for i in 0..100 {
            let s = format!("test {}", i);
            match processor.process(&[s.as_bytes().to_vec()]).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        }

        Ok(())
    }

    async fn stop_date(&self, symbols: &[String]) -> Result<(), Error> {
        println!("MockDataIntegration stop_date");
        println!("Stop data for symbols: {:#?}", symbols);
        Ok(())
    }

    async fn stop_all_date(&self) -> Result<(), Error> {
        println!("MockDataIntegration stop_all_date");

        Ok(())
    }
}
