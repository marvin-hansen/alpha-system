use crate::DataIntegrationTrait;
use binance_data_integration::ImsBinanceDataIntegration;
use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

const ID: &str = "BinanceDataIntegration";

#[derive(Debug, Default, Clone, Copy)]
pub struct BinanceDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl BinanceDataIntegration {
    pub fn new() -> Self {
        let binance_data_integration = ImsBinanceDataIntegration::new();
        Self {
            integration: binance_data_integration,
        }
    }
}

impl DataIntegrationTrait for BinanceDataIntegration {
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
