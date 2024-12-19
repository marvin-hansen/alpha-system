use crate::DataIntegrationTrait;
use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
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

#[derive(Default, DataIntegration)]
pub struct BinanceCoinFuturesTestnetDataIntegration {
    integration_id: ExchangeDataIntegrationID,
    integration: ImsBinanceCoinFuturesDataIntegration,
}

impl BinanceCoinFuturesTestnetDataIntegration {
    pub fn new() -> Self {
        Self {
            integration_id: ExchangeDataIntegrationID::BinanceCoinFuturesTestnetData,
            integration: ImsBinanceCoinFuturesDataIntegration::testnet(),
        }
    }
}
