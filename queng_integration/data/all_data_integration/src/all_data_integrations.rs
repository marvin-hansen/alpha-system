use crate::binance_data_integration::BinanceDataIntegration;
use crate::mock_data_implementation::MockDataIntegration;
use enum_dispatch::enum_dispatch;
use std::fmt::Error;

#[enum_dispatch(DataIntegration)]
pub trait TraitDataIntegration {
    async fn run(&self) -> Result<u32, Error>;
}

#[enum_dispatch]
pub enum DataIntegration {
    MockDataIntegration,
    BinanceDataIntegration,
}
