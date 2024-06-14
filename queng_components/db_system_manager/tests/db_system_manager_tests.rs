use common::prelude::{ClickHouseConfig, EnvironmentType, ServiceID};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use db_system_manager::SystemDBManager;
use dns_manager::DnsManager;
use env_utils::prelude::EnvUtil;
use smdb_specs::smdb_service_config;
use std::env;

async fn setup_env() {
    // Do the initial setup
    // Set the environment variable.
    env::set_var("ENV", "CI");
    // Internal CI DNS server.
    env::set_var("DNS_SERVER", "9.9.9.9");
}

// #[tokio::test]
async fn test_new() {
    // Setup env variables.
    setup_env().await;

    // Initialize the test environment to ensure all containers are up and running.
    let mut ci_env = EnvUtil::new();
    ci_env.setup_ci().await.expect("Failed to setup test env");

    // Build & configure components for contextual autoconfiguration.
    // Context manager determines the environment type.
    let ctxm = CtxManager::new();
    assert_eq!(ctxm.env_type(), EnvironmentType::CI);

    // Build & configure components for DNS autoconfiguration relative to the environment type.
    let dnm = DnsManager::new(&ctxm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    // Configure manager for context aware auto configuration.
    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config(), &ctxm, &dnm);
    assert_eq!(config_manager.get_svc_id(), ServiceID::SMDB);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CI);

    let clickhouse_config = ClickHouseConfig::new(
        "127.0.0.1".to_string(),
        9000,
        "default".to_string(),
        "".to_string(),
        "default".to_string(),
    );

    // Create DB component
    let sdbm = SystemDBManager::new(&clickhouse_config).await;
    assert!(sdbm.is_ok());

    // Unwrap
    let dbm = sdbm.unwrap();

    // Double check if connection is open
    assert!(dbm.is_open().await);
}
