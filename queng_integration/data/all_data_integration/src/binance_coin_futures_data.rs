use crate::DataIntegrationTrait;
use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use common_errors::MessageProcessingError;
use common_ims::ExchangeDataIntegrationID;
use std::collections::HashSet;
use std::fmt::Error;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

#[derive(Default)]
pub struct BinanceCoinFuturesDataIntegration {
    integration_id: ExchangeDataIntegrationID,
    integration: ImsBinanceCoinFuturesDataIntegration,
}

impl BinanceCoinFuturesDataIntegration {
    pub fn new() -> Self {
        Self {
            integration_id: ExchangeDataIntegrationID::BinanceCoinFuturesData,
            integration: ImsBinanceCoinFuturesDataIntegration::new(),
        }
    }
}

impl DataIntegrationTrait for BinanceCoinFuturesDataIntegration {
    async fn id(&self) -> Result<String, Error> {
        Ok(self.integration_id.to_string())
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

    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_trade_data(symbols).await
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

    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        self.integration.stop_ohlcv_data(symbols).await
    }

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        self.integration.stop_all_ohlcv_data().await
    }

    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
        self.integration.get_exchange_symbols().await
    }

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
        self.integration.validate_symbols(symbols).await
    }
}
