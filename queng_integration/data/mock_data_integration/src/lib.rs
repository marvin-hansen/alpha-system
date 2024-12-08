use common_errors::MessageProcessingError;
use std::sync::Arc;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

#[derive(Debug, Default, Clone, Copy)]
pub struct ImsMockDataIntegration;

impl ImsMockDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImsDataIntegration for ImsMockDataIntegration {
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        println!("MockDataIntegration start_data");
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

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        println!("MockDataIntegration stop_all_data");

        Ok(())
    }

    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        _processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        println!("MockDataIntegration start_trade_data");
        println!("Start data for symbols: {:#?}", symbols);
        Ok(())
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        println!("MockDataIntegration stop_all_data");

        Ok(())
    }

    async fn validate_symbols(&self, _symbols: &[String]) -> Result<bool, MessageProcessingError> {
        Ok(true)
    }
}
