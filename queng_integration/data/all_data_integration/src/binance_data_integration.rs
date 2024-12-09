use crate::DataIntegrationTrait;
use binance_spot_data_integration::ImsBinanceSpotDataIntegration;
use common_errors::MessageProcessingError;
use std::fmt::Error;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

const ID: &str = "BinanceDataIntegration";

#[derive(Default)]
pub struct BinanceSpotDataIntegration {
    integration: ImsBinanceSpotDataIntegration,
}

impl BinanceSpotDataIntegration {
    pub fn new() -> Self {
        let binance_data_integration = ImsBinanceSpotDataIntegration::new();
        Self {
            integration: binance_data_integration,
        }
    }
}

impl DataIntegrationTrait for BinanceSpotDataIntegration {
    async fn id(&self) -> Result<String, Error> {
        Ok(ID.to_string())
    }

    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_trade_data(symbols, processor).await
    }

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_trade_data().await
    }

    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_ohlcv_data(symbols, processor).await
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_ohlcv_data().await
    }

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        self.integration.validate_symbols(symbols).await
    }
}
