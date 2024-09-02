use config_manager::CfgManager;
use std::env;

use cmdb_specs::cmdb_service_config;
use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use ctx_manager::CtxManager;
use dbgw_specs::dbgw_service_config;
use dns_manager::DnsManager;
use smdb_specs::smdb_service_config;

#[tokio::test]
async fn test_get_cmdb_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);

    let dnm = DnsManager::new(&ctm).await;
    let cfg_manager = CfgManager::new(ServiceID::CMDB, cmdb_service_config(), &ctm, &dnm).await;

    let host = cfg_manager.get_svc_host_port().await.unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 7070));
}

#[tokio::test]
async fn test_get_smdb_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);

    let dnm = DnsManager::new(&ctm).await;
    let cfg_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config(), &ctm, &dnm).await;
    let host = cfg_manager.get_svc_host_port().await.unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 7070));
}

#[tokio::test]
async fn test_get_dbgw_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);

    let dnm = DnsManager::new(&ctm).await;
    let cfg_manager = CfgManager::new(ServiceID::DBGW, dbgw_service_config(), &ctm, &dnm).await;

    let host = cfg_manager.get_svc_host_port().await.unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 9090));
}
