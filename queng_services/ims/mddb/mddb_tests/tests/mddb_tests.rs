use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::prelude::DockerUtil;
use mddb_client::MDDBClient;
use metadata_import::MetadataImportManager;
use service_import::ServiceImportManager;
use service_utils::{ServiceUtil, ServiceWaitStrategy};
use std::time::Duration;

#[tokio::test]
async fn test_mddb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.get_or_start_container_config(&pg_container_config);
    dbg!(&result);
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();

    // Start service util
    let res = ServiceUtil::with_debug().await;
    dbg!(&res);
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    // Get config manger for automatic configuration
    let config_manager = svc_util.config_manager();

    let env_type = config_manager.env_type();

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

    // Here we assume you have a KaikoProxy service running locally;
    // On CI, we have to start the proxy service.
    if env_type == EnvironmentType::CI {
        // Start KaikoProxy proxy service, which is required for metadata import.
        let kaiko_service_id = ServiceID::KaikoProxy;
        let kaiko_wait_strategy = ServiceWaitStrategy::HttpHealthCheck(
            "http://localhost:8083/health".to_string(),
            Duration::from_secs(60),
        );
        let result = svc_util
            .start_service(&kaiko_service_id, &kaiko_wait_strategy)
            .await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    //Determine workflow for metadata import
    let meta_data_import_manager = MetadataImportManager::with_debug().await;

    // Import a sample of 50 metadata records for each type
    let workflow = meta_data_import_manager
        .determine_workflow(Some(50))
        .await
        .expect("Failed to determine workflow");

    dbg!(&workflow);

    // Execute workflow
    meta_data_import_manager
        .execute_workflow(&workflow)
        .await
        .expect("Failed to execute workflow");

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

    // Start MDDB service - depends on SMDB
    let service_id = ServiceID::MDDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    let (mddb_host, mddb_port) = config_manager
        .get_mddb_host_port()
        .await
        .expect("Failed to get MDDB host");
    dbg!(&mddb_host);
    dbg!(&mddb_port);

    // Construct MDDB client
    let mddb_client = MDDBClient::new(mddb_host, mddb_port)
        .await
        .expect("Failed to create MDDB client");

    // Test MDDB Assets metadata methods.

    // Test count_assets
    let result = mddb_client.count_assets().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 50);

    // Test check_if_asset_id_exists - success case i,e, exists
    let exists_id = "42";
    let result = mddb_client.check_if_asset_id_exists(exists_id).await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(exists);

    // Test check_if_asset_id_exists - Fail case i,e, does not exists.
    let does_not_exists_id = "zztopxyz_non_exist";
    let result = mddb_client
        .check_if_asset_id_exists(does_not_exists_id)
        .await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(!exists);

    // Test get_asset - success case
    let result = mddb_client.get_asset("42").await;
    assert!(result.is_ok());
    let asset = result.unwrap();
    assert!(asset.is_some());

    // Test get_asset - fail case
    let result = mddb_client.get_asset("zztopxyz_non_exist").await;
    assert!(result.is_err());

    // Test get_all_assets - success case
    let result = mddb_client.get_all_assets().await;
    assert!(result.is_ok());
    let assets = result.unwrap();
    assert!(!assets.is_empty());
    let len = assets.len();
    assert_eq!(len, 50);

    // Stop and remove container
    let result = docker_util.stop_container(&pg_container_id);
    dbg!(&result);
    assert!(result.is_ok());
}
