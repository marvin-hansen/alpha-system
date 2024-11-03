use container_specs_postgres::postgres_db_container_config;
use docker_utils::prelude::DockerUtil;
use service_import::ServiceImportManager;

// Somehow tests seem to be executed or sorted in alphabetical order,
// so make sure that the setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_check_if_already_imported() {
    let service_import_manager = ServiceImportManager::with_test_and_debug().await;

    let not_imported = service_import_manager.check_if_already_imported().await;
    assert!(!not_imported);

    service_import_manager
        .import_services()
        .await
        .expect("Failed to import services");

    let imported = service_import_manager.check_if_already_imported().await;
    assert!(imported);
}

#[tokio::test]
async fn test_import_services() {
    let service_import_manager = ServiceImportManager::with_test_and_debug().await;

    let not_imported = service_import_manager.check_if_already_imported().await;
    assert!(!not_imported);

    let zero_count = service_import_manager.count_db_services().await;
    assert_eq!(zero_count, 0);

    service_import_manager
        .import_services()
        .await
        .expect("Failed to import services");

    let imported = service_import_manager.check_if_already_imported().await;
    assert!(imported);

    let not_zero_count = service_import_manager.count_db_services().await;
    assert_ne!(zero_count, not_zero_count);
}

#[tokio::test]
async fn test_count_db_services() {
    let service_import_manager = ServiceImportManager::with_test_and_debug().await;

    let not_imported = service_import_manager.check_if_already_imported().await;
    assert!(!not_imported);

    let zero_count = service_import_manager.count_db_services().await;
    assert_eq!(zero_count, 0);

    service_import_manager
        .import_services()
        .await
        .expect("Failed to import services");

    let not_zero_count = service_import_manager.count_db_services().await;
    assert_ne!(zero_count, not_zero_count);
}
