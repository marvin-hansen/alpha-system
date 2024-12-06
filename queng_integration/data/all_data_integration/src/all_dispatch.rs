use crate::vex_data_integration::VexDataIntegration;
use crate::{BinanceDataIntegration, MockDataIntegration};
use enum_dispatch::enum_dispatch;
use std::fmt::Error;
use trait_data_integration::EventProcessor;

#[enum_dispatch]
#[derive(Clone, Copy)]
pub enum DataIntegration {
    BinanceDataIntegration,
    MockDataIntegration,
    VexDataIntegration,
}

#[enum_dispatch(DataIntegration)]
#[trait_variant::make(DataIntegrationTrait: Send)]
pub trait LocalDataIntegrationTrait {
    async fn id(&self) -> Result<String, Error>;

    async fn start_trade_data<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static;

    async fn stop_all_trade_data(&self) -> Result<(), Error>;

    async fn start_ohlcv_data<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static;

    async fn stop_all_ohlcv_data(&self) -> Result<(), Error>;
}
