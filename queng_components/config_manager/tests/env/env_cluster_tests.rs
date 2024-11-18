use common_config::ServiceID;
use common_env::EnvironmentType;
use config_manager::CfgManager;
use smdb_specs::smdb_service_config;
use std::env;

// In a cluster environment, DNS server must be set to resolve internal DNS service names.

#[tokio::test]
async fn test_env_type() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
}
