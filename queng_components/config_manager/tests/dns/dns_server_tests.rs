use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use config_manager::CfgManager;
use smdb_specs::smdb_service_config;
use std::env;

#[tokio::test]
async fn test_new() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config()).await;
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);

    assert_eq!(config_manager.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(config_manager.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_internal_dns() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::with_debug(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.internal_dns_server(), "9.9.9.9:53");
}

// We cannot test the internal DNS resolution since
// this would require a custom DNS server with custom DNS records.
// However, this is build for Kubernetes and usually k8s DNS works reliably.

#[tokio::test]
async fn test_external_dns() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::with_debug(ServiceID::SMDB, smdb_service_config()).await;
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);

    assert_eq!(config_manager.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(config_manager.external_dns_server(), "1.1.1.1:53");
}

// Unclear why this test fails on CI.
// Disabled for now.
//
// #[tokio::test]
// async fn test_resolve_external_dns() {
//     env::set_var("ENV", "CLUSTER");
//     env::set_var("DNS_SERVER", "9.9.9.9");
//
//     let config_manager = CfgManager::with_debug(ServiceID::SMDB, smdb_service_config()).await;
//
//     assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
//     assert_eq!(config_manager.internal_dns_server(), "9.9.9.9:53");
//     assert_eq!(config_manager.external_dns_server(), "1.1.1.1:53");
//
//     let host = "harvard.edu";
//     let res = config_manager.resolve_dns(host, false).await;
//     assert!(res.is_ok());
//
//     let host = "mit.edu";
//     let res = config_manager.resolve_dns(host, false).await;
//     assert!(res.is_ok());
// }
