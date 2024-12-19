use crate::DataIntegrationTrait;
use binance_usd_futures_data_integration::ImsBinanceUsdFuturesDataIntegration;
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
pub struct BinanceUsdFuturesDataIntegration {
    integration_id: ExchangeDataIntegrationID,
    integration: ImsBinanceUsdFuturesDataIntegration,
}

impl BinanceUsdFuturesDataIntegration {
    pub fn new() -> Self {
        Self {
            integration_id: ExchangeDataIntegrationID::BinanceUsdFuturesData,
            integration: ImsBinanceUsdFuturesDataIntegration::new(),
        }
    }
}

#[derive(Default, DataIntegration)]
pub struct BinanceUsdFuturesTestnetDataIntegration {
    integration_id: ExchangeDataIntegrationID,
    integration: ImsBinanceUsdFuturesDataIntegration,
}

impl BinanceUsdFuturesTestnetDataIntegration {
    pub fn new() -> Self {
        Self {
            integration_id: ExchangeDataIntegrationID::BinanceUsdFuturesTestnetData,
            integration: ImsBinanceUsdFuturesDataIntegration::new(),
        }
    }
}
