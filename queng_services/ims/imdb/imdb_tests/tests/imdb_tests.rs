use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::prelude::DockerUtil;
use imdb_client::IMDBClient;
use service_import::ServiceImportManager;
use service_utils::{ServiceUtil, ServiceWaitStrategy};
use std::time::Duration;

#[tokio::test]
async fn test_imdb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");
    // Start service util
    let res = ServiceUtil::with_debug().await;
    dbg!(&res);
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    // Get config manger for automatic configuration
    let config_manager = svc_util.config_manager();

    let env_type = config_manager.env_type();

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.get_or_start_container_config(&pg_container_config);
    dbg!(&result);
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();

    // Test if service data is already imported in the DB; if not, do so.
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

    // Wait for services to be ready
    let wait_strategy = if env_type == EnvironmentType::LOCAL {
        ServiceWaitStrategy::Duration(Duration::from_millis(250))
    } else {
        ServiceWaitStrategy::Duration(Duration::from_millis(500))
    };

    // Start DBGW service - depends on Database
    let service_id = ServiceID::DBGW;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    dbg!(&result);
    assert!(result.is_ok());

    // Start SMDB service - depends on DBGW
    let service_id = ServiceID::SMDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Start IMDB service - depends on SMDB and DBGW
    let service_id = ServiceID::IMDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Configure IMDB client
    let (host, port) = config_manager
        .get_imdb_host_port()
        .await
        .expect("Failed to get MDDB host");
    dbg!(&host);
    dbg!(&port);

    // Construct IMDB client
    let client = IMDBClient::new(host, port)
        .await
        .expect("Failed to create IMDB client");

    // Test IMDB service with IMDB client
    test_imdb_integrations(&client).await;

    // Stop and remove container
    let result = docker_util.stop_container(&pg_container_id);
    dbg!(&result);
    assert!(result.is_ok());
}

async fn test_imdb_integrations(_client: &IMDBClient) {
    // Write acceptance tests
}
