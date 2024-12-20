use binance_core_data_integration::ImsBinanceDataIntegration;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use std::collections::HashSet;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

// LIVE API

const API_BASE_URL: &str = "https://api.binance.com/api/v3";
const API_WSS_URL: &str = "wss://stream.binance.com:9443/ws";

// TESTNET API
// https://www.binance.com/en/support/faq/how-to-test-my-functions-on-binance-testnet-ab78f9a1b8824cf0a106b4229c76496d
const TESTNET_API_BASE_URL: &str = "https://testnet.binance.vision/api/v3";
const TESTNET_API_WSS_URL: &str = "wss://testnet.binance.vision/ws";

#[derive(Default)]
pub struct ImsBinanceSpotDataIntegration {
    integration: ImsBinanceDataIntegration,
}

impl ImsBinanceSpotDataIntegration {
    pub fn new() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(API_BASE_URL, API_WSS_URL),
        }
    }

    pub fn testnet() -> Self {
        Self {
            integration: ImsBinanceDataIntegration::new(TESTNET_API_BASE_URL, TESTNET_API_WSS_URL),
        }
    }
}

impl ImsDataIntegration for ImsBinanceSpotDataIntegration {
    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        self.integration.get_exchange_symbols().await
    }

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        self.integration.validate_symbols(symbols).await
    }
}

impl ImsTradeDataIntegration for ImsBinanceSpotDataIntegration {
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

    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_trade_data(symbols).await
    }

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_trade_data().await
    }
}

impl ImsOhlcvDataIntegration for ImsBinanceSpotDataIntegration {
    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration
            .start_ohlcv_data(symbols, time_resolution, processor)
            .await
    }

    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_ohlcv_data(symbols).await
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_ohlcv_data().await
    }
}
