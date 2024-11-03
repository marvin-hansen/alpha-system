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

    let service_import_manager = ServiceImportManager::with_debug().await;
    let imported = service_import_manager.check_if_already_imported().await;

    if !imported {
        service_import_manager
            .import_services()
            .await
            .expect("Failed to import services");
    }

    let imported = service_import_manager.check_if_already_imported().await;

    assert!(imported);
}
