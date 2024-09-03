use cmdb_specs::cmdb_service_config;
use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use config_manager::CfgManager;
use dbgw_specs::dbgw_service_config;
use qdgw_specs::qdgw_service_config;
use smdb_specs::smdb_service_config;

use std::env;

#[tokio::test]
async fn new_config_manager_smdb() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::with_debug(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.get_svc_id(), ServiceID::SMDB);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), smdb_service_config());
}

#[tokio::test]
async fn new_config_manager_cmdb() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::with_debug(ServiceID::CMDB, cmdb_service_config()).await;

    assert_eq!(config_manager.get_svc_id(), ServiceID::CMDB);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), cmdb_service_config());
}

#[tokio::test]
async fn new_config_manager_dbgw() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::with_debug(ServiceID::DBGW, dbgw_service_config()).await;

    assert_eq!(config_manager.get_svc_id(), ServiceID::DBGW);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), dbgw_service_config());
}

#[tokio::test]
async fn new_config_manager_qdgw() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::with_debug(ServiceID::QDGW, qdgw_service_config()).await;

    assert_eq!(config_manager.get_svc_id(), ServiceID::QDGW);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), qdgw_service_config());
}
