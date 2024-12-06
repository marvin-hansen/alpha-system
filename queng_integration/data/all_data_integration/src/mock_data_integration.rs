use crate::DataIntegrationTrait;
use mock_data_integration::ImsMockDataIntegration;
use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

const ID: &str = "MockDataIntegration";

#[derive(Debug, Default, Clone, Copy)]
pub struct MockDataIntegration {
    integration: ImsMockDataIntegration,
}

impl MockDataIntegration {
    pub fn new() -> Self {
        let mock_data_integration = ImsMockDataIntegration::new();
        Self {
            integration: mock_data_integration,
        }
    }
}

impl DataIntegrationTrait for MockDataIntegration {
    async fn id(&self) -> Result<String, Error> {
        Ok(ID.to_string())
    }

    async fn start_trade_data<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_trade_data(symbols, processor).await
    }

    async fn stop_all_trade_data(&self) -> Result<(), Error> {
        self.integration.stop_all_trade_data().await
    }

    async fn start_ohlcv_data<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_ohlcv_data(symbols, processor).await
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), Error> {
        self.integration.stop_all_ohlcv_data().await
    }
}
