use common_ims::IntegrationConfig;
use ims_data_binance_specs::binance_ims_data_integration_config;

/// This module provides a public function to return a vector of `IntegrationConfig` instances.
///
/// The function is used by the `integration_import` crate to build the database of integration
/// configurations.
//
#[must_use]
pub fn get_all_integration_configs() -> Vec<IntegrationConfig> {
    vec![binance_ims_data_integration_config()]
}
