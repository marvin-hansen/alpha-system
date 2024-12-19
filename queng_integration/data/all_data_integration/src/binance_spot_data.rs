use crate::DataIntegrationTrait;
use binance_spot_data_integration::ImsBinanceSpotDataIntegration;
use common_data_bar::TimeResolution;
use common_errors::MessageProcessingError;
use common_ims::ExchangeDataIntegrationID;
use data_integration_macro::DataIntegration;
use std::collections::HashSet;
use std::fmt::Error;
use std::sync::Arc;
use trait_data_integration::{
    EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration, ImsTradeDataIntegration,
};

#[derive(Default, DataIntegration)]
pub struct BinanceSpotDataIntegration {
    integration_id: ExchangeDataIntegrationID,
    integration: ImsBinanceSpotDataIntegration,
}

impl BinanceSpotDataIntegration {
    pub fn new() -> Self {
        Self {
            integration_id: ExchangeDataIntegrationID::BinanceSpotData,
            integration: ImsBinanceSpotDataIntegration::new(),
        }
    }
}

#[derive(Default, DataIntegration)]
pub struct BinanceSpotTestnetDataIntegration {
    integration_id: ExchangeDataIntegrationID,
    integration: ImsBinanceSpotDataIntegration,
}

impl BinanceSpotTestnetDataIntegration {
    pub fn new() -> Self {
        Self {
            integration_id: ExchangeDataIntegrationID::BinanceSpotTestnetData,
            integration: ImsBinanceSpotDataIntegration::testnet(),
        }
    }
}
