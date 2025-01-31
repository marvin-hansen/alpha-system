/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use config_import::ConfigImportManager;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;

// Somehow tests seem to be executed or sorted in alphabetical order,
// so make sure that the setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_import_portfolios() {
    let config_import_manager = ConfigImportManager::with_test_and_debug().await;

    let not_imported = config_import_manager.check_if_portfolios_imported().await;
    assert!(!not_imported);

    let zero_count = config_import_manager.count_db_portfolios().await;
    assert_eq!(zero_count, 0);

    config_import_manager
        .import_portfolio_configs()
        .await
        .expect("Failed to import portfolios");

    let imported = config_import_manager.check_if_portfolios_imported().await;
    assert!(imported);

    let not_zero_count = config_import_manager.count_db_portfolios().await;
    assert_ne!(zero_count, not_zero_count);
}

#[tokio::test]
pub async fn test_check_if_portfolios_already_imported() {
    let config_import_manager = ConfigImportManager::with_test_and_debug().await;

    let not_imported = config_import_manager.check_if_portfolios_imported().await;
    assert!(!not_imported);

    config_import_manager
        .import_portfolio_configs()
        .await
        .expect("Failed to import portfolios");

    let imported = config_import_manager.check_if_portfolios_imported().await;
    assert!(imported);
}

#[tokio::test]
async fn test_count_db_portfolios() {
    let config_import_manager = ConfigImportManager::with_test_and_debug().await;

    let zero_count = config_import_manager.count_db_portfolios().await;
    assert_eq!(zero_count, 0);

    config_import_manager
        .import_portfolio_configs()
        .await
        .expect("Failed to import portfolios");

    let not_zero_count = config_import_manager.count_db_portfolios().await;
    assert_ne!(zero_count, not_zero_count);
}
