use cmdb_specs::cmdb_service_config;
use common_config::ServiceConfig;
use dbgw_specs::dbgw_service_config;
use imdb_specs::imdb_service_config;
use mddb_specs::mddb_service_config;
use smdb_specs::smdb_service_config;

#[must_use]
pub fn get_all_service_specs() -> Vec<ServiceConfig> {
    // Update tests if you add more service specs.
    Vec::from([
        cmdb_service_config(),
        dbgw_service_config(),
        imdb_service_config(),
        mddb_service_config(),
        smdb_service_config(),
    ])
}
