use crate::BinanceUsdFuturesDataIntegration;
use crate::{BinanceCoinFuturesDataIntegration, BinanceSpotTestnetDataIntegration};
use crate::{
    BinanceCoinFuturesTestnetDataIntegration, BinanceSpotDataIntegration,
    BinanceUsdFuturesTestnetDataIntegration,
};
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use enum_dispatch::enum_dispatch;
use std::collections::HashSet;
use std::fmt::Error;
use std::sync::Arc;
use trait_data_integration::EventProcessor;

#[enum_dispatch]
pub enum DataIntegration {
    // These Enum values must match exactly the name of the struct type of the integration.
    // Binance Live
    BinanceSpotDataIntegration,
    BinanceUsdFuturesDataIntegration,
    BinanceCoinFuturesDataIntegration,
    // Binance Testnet
    BinanceSpotTestnetDataIntegration,
    BinanceUsdFuturesTestnetDataIntegration,
    BinanceCoinFuturesTestnetDataIntegration,
    //
}

#[enum_dispatch(DataIntegration)]
#[trait_variant::make(DataIntegrationTrait: Send)]
pub trait LocalDataIntegrationTrait {
    async fn id(&self) -> Result<String, Error>;

    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static;

    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>;

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError>;

    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static;

    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>;

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>;

    async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError>;

    async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError>;
}
