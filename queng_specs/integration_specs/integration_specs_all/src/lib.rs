use common_ims::prelude::IntegrationConfig;
use ims_data_binance_specs::ims_data_integration_binance_config;

/// This module provides a public function to return a vector of IntegrationConfig instances.
///
/// The function is used by the integration_import crate to build the database of integration
/// configurations.
//
pub fn get_all_integration_configs() -> Vec<IntegrationConfig> {
    vec![ims_data_integration_binance_config()]
}
