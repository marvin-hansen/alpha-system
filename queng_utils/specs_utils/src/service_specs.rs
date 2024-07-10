use cmdb_specs::cmdb_service_config;
use common::prelude::ServiceConfig;
use dbgw_specs::dbgw_service_config;
use mddb_specs::mddb_service_config;
use qdgw_specs::qdgw_service_config;
use smdb_specs::smdb_service_config;
use vex_specs::vex_service_config;

pub fn get_all_service_specs() -> Vec<ServiceConfig> {
    // Update tests if you add more service specs.
    Vec::from([
        cmdb_service_config(),
        dbgw_service_config(),
        mddb_service_config(),
        qdgw_service_config(),
        smdb_service_config(),
        vex_service_config(),
    ])
}
