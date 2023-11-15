use std::env;

use common::prelude::{EnvironmentType, ServiceConfig, ServiceID};
use components::prelude::{CfgManager, CtxManager};
use specs::prelude::{cmdb_service_config, dbgw_service_config, smdb_service_config};

#[test]
fn new_config_manager_default() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::Default, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::Default);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(
        config_manager.main_config(),
        ServiceConfig::default().main_config()
    );
    assert_eq!(config_manager.svc_config(), ServiceConfig::default());
}

#[test]
fn new_config_manager_smdb() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::SMDB, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::SMDB);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(
        config_manager.main_config(),
        smdb_service_config().main_config()
    );
    assert_eq!(config_manager.svc_config(), smdb_service_config());
}

#[test]
fn new_config_manager_cmdb() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::CMDB, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::CMDB);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(
        config_manager.main_config(),
        cmdb_service_config().main_config()
    );
    assert_eq!(config_manager.svc_config(), cmdb_service_config());
}

#[test]
fn new_config_manager_dbgw() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::DBGW, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::DBGW);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(
        config_manager.main_config(),
        dbgw_service_config().main_config()
    );
    assert_eq!(config_manager.svc_config(), dbgw_service_config());
}
