use std::env;

use common::prelude::{EnvironmentType, ServiceConfig, ServiceID};
use components::prelude::{CfgManager, CtxManager};
use specs::prelude::{
    cmdb_service_config, db_config_ci, db_config_cluster, db_config_local, dbgw_service_config,
    qdgw_service_config, smdb_service_config,
};

#[test]
fn new_config_manager_default() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::Default, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::Default);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
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
    assert_eq!(config_manager.svc_config(), dbgw_service_config());
}

#[test]
fn new_config_manager_qdgw() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let config_manager = CfgManager::new(ServiceID::QDGW, &ctx);

    assert_eq!(config_manager.svc(), ServiceID::QDGW);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.svc_config(), qdgw_service_config());
}

#[test]
fn test_get_db_config_local() {
    env::set_var("ENV", "LOCAL");

    let ctx = CtxManager::new();
    let cfg_manager = CfgManager::new(ServiceID::DBGW, &ctx);
    let db_config = cfg_manager.get_db_config();
    assert_eq!(db_config, db_config_local());
}

#[test]
fn test_get_db_config_ci() {
    env::set_var("ENV", "CI");

    let ctx = CtxManager::new();
    let cfg_manager = CfgManager::new(ServiceID::DBGW, &ctx);
    let db_config = cfg_manager.get_db_config();
    assert_eq!(db_config, db_config_ci());
}

#[test]
fn test_get_db_config_cluster() {
    env::set_var("ENV", "CLUSTER");

    let ctx = CtxManager::new();
    let cfg_manager = CfgManager::new(ServiceID::DBGW, &ctx);
    let db_config = cfg_manager.get_db_config();
    assert_eq!(db_config, db_config_cluster());
}
