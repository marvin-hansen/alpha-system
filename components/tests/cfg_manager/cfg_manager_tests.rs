use std::env;

use common::prelude::{EnvironmentType, ServiceID};
use components::cfg_manager::CfgManager;
use components::prelude::CtxManager;
use specs::memgraph::memgraph_service_config;

#[test]
fn new_config_manager() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::MEMGRAPH, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::MEMGRAPH);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(
        config_manager.main_config(),
        &memgraph_service_config().main_config()
    );
    assert_eq!(config_manager.svc_config(), &memgraph_service_config());
}
