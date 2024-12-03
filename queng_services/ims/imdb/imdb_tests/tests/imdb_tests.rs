use common_config::ServiceID;
use common_ims::ExchangeID;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use imdb_client::IMDBClient;
use integration_import::IntegrationImportManager;
use service_import::ServiceImportManager;
use service_utils::ServiceUtil;
use std::time::Duration;
use wait_utils::WaitStrategy;

async fn get_service_wait_strategy(host: String, port: u16) -> WaitStrategy {
    let url = format!("http://{host}:{port}");
    WaitStrategy::WaitForGrpcHealthCheck(url, 10)
}

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

    // Test if integration data has already been imported in the DB; if not, do so.
    let integration_import_manager = IntegrationImportManager::with_debug().await;

    let imported = integration_import_manager
        .check_if_integrations_imported()
        .await;
    if !imported {
        integration_import_manager
            .import_integration_configs()
            .await
            .expect("Failed to import integrations");
    }

    let imported = integration_import_manager
        .check_if_integrations_imported()
        .await;
    assert!(imported);

    dbg!("Start DBGW service - depends on Database");
    let service_id = ServiceID::DBGW;
    let (host, port) = config_manager
        .get_dbgw_host_port()
        .await
        .expect("Failed to get host and port for DBGW");
    let wait_strategy = get_service_wait_strategy(host, port).await;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    dbg!(&result);
    assert!(result.is_ok());

    dbg!("Start SMDB service - depends on DBGW");
    let service_id = ServiceID::SMDB;
    let (host, port) = config_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");
    let wait_strategy = get_service_wait_strategy(host, port).await;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    dbg!("Start IMDB - depends on SMDB");
    let service_id = ServiceID::IMDB;
    let (host, port) = config_manager
        .get_imdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");
    let wait_strategy = get_service_wait_strategy(host, port).await;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Configure IMDB client
    let (host, port) = config_manager
        .get_imdb_host_port()
        .await
        .expect("Failed to get MDDB host");
    dbg!(&host);
    dbg!(&port);

    assert!(!host.is_empty());
    assert!(port > 0);

    // Wait 0.5 second before starting the IMDB client
    tokio::time::sleep(Duration::from_millis(500)).await;

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

async fn test_imdb_integrations(client: &IMDBClient) {
    // Test count_integrations
    let res = client.count_integrations().await;
    dbg!(&res);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert!(count > 0);

    // Test check_if_integration_exists
    let integration_id = "ims-data-binance".to_string();
    let res = client
        .check_if_integration_exists(integration_id.clone())
        .await;
    dbg!(&res);
    assert!(res.is_ok());

    let exists = res.unwrap();
    assert!(exists);

    // Test check_if_integration_online
    let integration_id = "ims-data-binance".to_string();
    let res = client
        .check_if_integration_online(integration_id.clone())
        .await;
    assert!(res.is_ok());

    // By default, all integrations are offline in the DB.
    let online = res.unwrap();
    assert!(!online);

    // Test get_integration
    let integration_id = "ims-data-binance".to_string();
    let res = client.get_integration(integration_id.clone()).await;
    assert!(res.is_ok());

    let integration = res.unwrap();
    assert!(integration.is_some());

    let integration = integration.unwrap();
    assert_eq!(integration.integration_id(), &integration_id);

    // Test get_all_integrations
    let res = client.get_all_integrations().await;
    assert!(res.is_ok());

    let integrations = res.unwrap();
    assert!(!integrations.is_empty());

    // Test get_all_integrations_by_exchange
    let exchange_id = ExchangeID::Binance;
    let res = client.get_all_integrations_by_exchange(exchange_id).await;
    assert!(res.is_ok());

    let integrations = res.unwrap();
    assert!(!integrations.is_empty());

    // Test get_all_online_integrations
    // By default, all integrations are offline in the DB.
    // Therefore, this is expected to return an empty vector.
    let res = client.get_all_online_integrations().await;
    assert!(res.is_ok());

    let integrations = res.unwrap();
    assert_eq!(integrations.len(), 0);

    // get_all_offline_integrations
    let res = client.get_all_offline_integrations().await;
    assert!(res.is_ok());

    let integrations = res.unwrap();
    assert!(!integrations.is_empty());

    // Test set_integration_online
    let integration_id = "ims-data-binance".to_string();

    // Test if integration is offline, which it is by default.
    let res = client
        .check_if_integration_online(integration_id.clone())
        .await;
    assert!(res.is_ok());
    // By default, all integrations are offline in the DB.
    let online = res.unwrap();
    assert!(!online);

    // Set integration online
    let res = client.set_integration_online(integration_id.clone()).await;
    assert!(res.is_ok());

    // Test if integration is online
    let res = client
        .check_if_integration_online(integration_id.clone())
        .await;
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Test set_integration_offline
    let res = client.set_integration_offline(integration_id.clone()).await;
    assert!(res.is_ok());

    // Test if integration is offline
    let res = client
        .check_if_integration_online(integration_id.clone())
        .await;
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);
}
