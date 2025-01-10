use binance_data_specs::binance_ims_data_integration_config;
use common_ims::{ExchangeID, IntegrationConfig};

/// This module provides a public function to return a vector of `IntegrationConfig` instances.
///
/// The function is used by the `integration_import` crate to build the database of integration
/// configurations.
//
#[must_use]
pub fn get_all_integration_configs() -> Vec<IntegrationConfig> {
    vec![
        binance_ims_data_integration_config(ExchangeID::BinanceSpot),
        binance_ims_data_integration_config(ExchangeID::BinanceSpotTestnet),
        binance_ims_data_integration_config(ExchangeID::BinanceCoinMarginFuture),
        binance_ims_data_integration_config(ExchangeID::BinanceCoinMarginFutureTestnet),
        binance_ims_data_integration_config(ExchangeID::BinanceUsdMarginFuture),
        binance_ims_data_integration_config(ExchangeID::BinanceUsdMarginFutureTestnet),
    ]
}
