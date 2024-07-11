use common::prelude::{ClickHouseConfig, EnvironmentType};
use ctx_manager::CtxManager;
use db_system_manager::SystemDBManager;
use env_utils::prelude::EnvUtil;
use std::env;

async fn setup_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");
}

#[tokio::test]
async fn test_new() {
    // Setup env variables.
    setup_env().await;

    // Initialize the test environment to ensure all containers are up and running.
    let mut ci_env = EnvUtil::new().await.expect("Failed to get EnvUtil");
    ci_env.setup_ci().await.expect("Failed to setup test env");

    // Build & configure components for contextual autoconfiguration.
    // Context manager determines the environment type.
    let ctxm = CtxManager::new();
    assert_eq!(ctxm.env_type(), EnvironmentType::CLUSTER);

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
