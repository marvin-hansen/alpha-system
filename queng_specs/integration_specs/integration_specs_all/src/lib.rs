/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_ims::{ExchangeID, IntegrationConfig};
use shared_service_specs::ims_data_integration_config;

/// This module provides a public function to return a vector of `IntegrationConfig` instances.
///
/// The function is used by the `integration_import` crate to build the database of integration
/// configurations.
//
#[must_use]
pub fn get_all_integration_configs() -> Vec<IntegrationConfig> {
    vec![
        ims_data_integration_config(ExchangeID::BinanceSpot),
        ims_data_integration_config(ExchangeID::BinanceSpotTestnet),
        ims_data_integration_config(ExchangeID::BinanceCoinMarginFuture),
        ims_data_integration_config(ExchangeID::BinanceCoinMarginFutureTestnet),
        ims_data_integration_config(ExchangeID::BinanceUsdMarginFuture),
        ims_data_integration_config(ExchangeID::BinanceUsdMarginFutureTestnet),
    ]
}
