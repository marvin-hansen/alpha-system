use crate::services::ims_data::shared::ims_service_config;
use common::prelude::{ExchangeID, ServiceConfig, ServiceID};

pub fn ims_data_binance_config() -> ServiceConfig {
    ims_service_config(ExchangeID::Binance, ServiceID::ImsDataBinance)
}
