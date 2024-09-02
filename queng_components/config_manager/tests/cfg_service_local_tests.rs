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

    let service_id = ServiceID::CMDB;

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);

    let dnm = DnsManager::new(&ctm).await;
    let cfg_manager = CfgManager::new(service_id, cmdb_service_config(), &ctm, &dnm).await;

    let actual_host_port = cfg_manager.get_svc_host_port().await.unwrap();

    let expected_local_host = "0.0.0.0".to_string();
    let expected_local_port = 7070 + service_id.as_u16();
    assert_eq!(actual_host_port, (expected_local_host, expected_local_port));
}

#[tokio::test]
async fn test_get_smdb_host() {
    env::set_var("ENV", "LOCAL");

    let service_id = ServiceID::SMDB;

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);

    let dnm = DnsManager::new(&ctm).await;
    let cfg_manager = CfgManager::new(service_id, smdb_service_config(), &ctm, &dnm).await;
    let actual_host_port = cfg_manager.get_svc_host_port().await.unwrap();
    let expected_local_host = "0.0.0.0".to_string();
    let expected_local_port = 7070 + service_id.as_u16();
    assert_eq!(actual_host_port, (expected_local_host, expected_local_port));
}

#[tokio::test]
async fn test_get_dbgw_host() {
    env::set_var("ENV", "LOCAL");

    let service_id = ServiceID::DBGW;

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);

    let dnm = DnsManager::new(&ctm).await;
    let cfg_manager = CfgManager::new(service_id, dbgw_service_config(), &ctm, &dnm).await;

    let actual_host_port = cfg_manager.get_svc_host_port().await.unwrap();
    let expected_local_host = "0.0.0.0".to_string();
    let expected_local_port = 9090 + service_id.as_u16();
    assert_eq!(actual_host_port, (expected_local_host, expected_local_port));
}
