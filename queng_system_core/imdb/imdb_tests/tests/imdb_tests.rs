/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_ims::ExchangeID;
use config_manager::CfgManager;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use imdb_client::IMDBClient;
use imdb_client::ImdbClientTrait;
use integration_import::IntegrationImportManager;
use service_import::ServiceImportManager;
use service_utils::{ServiceStartConfig, ServiceUtil};
use std::time::Duration;

fn get_service_wait_strategy(host: String, port: u16) -> service_utils::WaitStrategy {
    let url = format!("http://{host}:{port}");
    service_utils::WaitStrategy::WaitForGrpcHealthCheck(url, 10)
}

fn get_service_start_config(program: &'static str, host: String, port: u16) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(program)
        .wait_strategy(get_service_wait_strategy(host, port))
        .build()
}

pub const ROOT_PATH: &str = "queng_system_core/imdb/imdb_tests/tests";

pub const BINARIES: [&str; 3] = ["dbgw", "smdb", "imdb"];

#[tokio::test]
async fn test_imdb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.get_or_start_container(&pg_container_config);
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();
    dbg!("✅ Postgres container ID: {pg_container_id} started");

    dbg!("Start service util");
    let res = ServiceUtil::with_debug(ROOT_PATH, Vec::from(BINARIES)).await;
    if res.is_err() {
        dbg!(&res);
    }
    assert!(res.is_ok());
    let svc_util = res.unwrap();
    dbg!("✅ service util started");

    dbg!("Start config manager");
    let config_manager = CfgManager::default_with_debug();
    dbg!("✅ config manager started");

    dbg!("Test if service data is already imported in the DB; if not, do so.");
    let service_import_manager = ServiceImportManager::with_debug().await;
    let imported = service_import_manager.check_if_already_imported().await;

    if !imported {
        dbg!("Import service data into the DB");
        service_import_manager
            .import_services()
            .await
            .expect("Failed to import services");
    }

    let imported = service_import_manager.check_if_already_imported().await;
    assert!(imported);
    dbg!("✅ Service data imported");

    // Test if integration data has already been imported in the DB; if not, do so.
    let integration_import_manager = IntegrationImportManager::with_debug().await;

    let imported = integration_import_manager
        .check_if_integrations_imported()
        .await;
    if !imported {
        dbg!("Import integration data into the DB");
        integration_import_manager
            .import_integration_configs()
            .await
            .expect("Failed to import integrations");
    }

    let imported = integration_import_manager
        .check_if_integrations_imported()
        .await;
    assert!(imported);
    dbg!("✅ Integration data imported");

    dbg!("Start DBGW service - depends on Database");
    let (host, port) = config_manager
        .get_dbgw_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    dbg!(&host);
    dbg!(&port);

    let dbgw_start_config = get_service_start_config("dbgw", host, port);
    let result = svc_util.start_service_from_config(dbgw_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ DBGW service started");

    dbg!("Start SMDB service - depends on DBGW");
    let (host, port) = config_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    dbg!(&host);
    dbg!(&port);

    let smdb_start_config = get_service_start_config("smdb", host, port);
    let result = svc_util.start_service_from_config(smdb_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ SMDB service started");

    dbg!("Start IMDB service - depends on SMDB");
    let (host, port) = config_manager
        .get_imdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    let imdb_start_config = get_service_start_config("imdb", host, port);
    let result = svc_util.start_service_from_config(imdb_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ IMDB service started");

    dbg!("Configure IMDB client");
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
    dbg!("✅ IMDB client started");

    // Test IMDB service with IMDB client
    test_imdb_integrations(&client).await;

    // Stop and remove container
    let delete_container = true;
    let result = docker_util.stop_container(&pg_container_id, delete_container);
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
    let integration_id = "binancespot-data".to_string();
    let res = client
        .check_if_integration_exists(integration_id.clone())
        .await;
    dbg!(&res);
    assert!(res.is_ok());

    let exists = res.unwrap();
    assert!(exists);

    // Test check_if_integration_online
    let integration_id = "binancespot-data".to_string();
    let res = client
        .check_if_integration_online(integration_id.clone())
        .await;
    assert!(res.is_ok());

    // By default, all integrations are offline in the DB.
    let online = res.unwrap();
    assert!(!online);

    // Test get_integration
    let integration_id = "binancespot-data".to_string();
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
    let exchange_id = ExchangeID::BinanceSpot;
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
    let integration_id = "binancespot-data".to_string();

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
