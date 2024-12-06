use crate::{BinanceDataIntegration, MockDataIntegration};
use common_errors::MessageProcessingError;
use enum_dispatch::enum_dispatch;
use std::fmt::Error;
use std::sync::Arc;
use trait_data_integration::EventProcessor;

#[enum_dispatch]
pub enum DataIntegration {
    BinanceDataIntegration,
    MockDataIntegration,
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

    async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError>;

    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static;

    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>;
}
