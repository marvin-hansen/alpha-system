/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use cmdb_client::CmdbClient;
use config_manager::CfgManager;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use service_import::ServiceImportManager;
use service_utils::*;

fn get_service_wait_strategy(host: String, port: u16) -> WaitStrategy {
    let url = format!("http://{host}:{port}");
    WaitStrategy::WaitForGrpcHealthCheck(url, 10)
}

fn get_service_start_config(program: &'static str, host: String, port: u16) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(program)
        .wait_strategy(get_service_wait_strategy(host, port))
        .build()
}

pub const ROOT_PATH: &str = "queng_system_core/cmdb/cmdb_tests/tests";

pub const BINARIES: [&str; 3] = ["dbgw", "smdb", "cmdb"];

#[tokio::test]
async fn test_cmdb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    dbg!("Start or reuse a test postgres database container");
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.setup_container(&pg_container_config);
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();
    format!("✅ Postgres container ID: {pg_container_id} started");

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

    dbg!("Start CMDB service - depends on SMDB and DBGW");
    let (host, port) = config_manager
        .get_cmdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    dbg!(&host);
    dbg!(&port);

    let cmdb_start_config = get_service_start_config("cmdb", host, port);
    let result = svc_util.start_service_from_config(cmdb_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ CMDB service started");

    dbg!("Configure CMDB client");
    let (cmdb_host, cmdb_port) = config_manager
        .get_cmdb_host_port()
        .await
        .expect("Failed to get SMDB host");
    dbg!(&cmdb_host);
    dbg!(&cmdb_port);

    // Construct CMDB client
    let cmdb_client = CmdbClient::new(cmdb_host, cmdb_port).await;
    dbg!("✅ CMDB client started");

    // Test create_portfolio_config  - Success!
    let portfolio_config = portfolio_specs::get_test_portfolio_config();
    let result = cmdb_client
        .create_portfolio_config(portfolio_config.clone())
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    // Test create_portfolio_config - Error: portfolio config already exists
    let result = cmdb_client
        .create_portfolio_config(portfolio_config.clone())
        .await;
    dbg!(&result);
    assert!(result.is_err());

    // Test read_portfolio_config_by_id - Success!
    let result = cmdb_client
        .read_portfolio_config_by_id(portfolio_config.portfolio_id() as u16)
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    // Test read_portfolio_config_by_id - Error: portfolio config does not exist
    let result = cmdb_client.read_portfolio_config_by_id(47).await;
    dbg!(&result);
    assert!(result.is_err());

    // Test read_all_portfolio_configs - Success!
    let result = cmdb_client.read_all_portfolio_configs().await;
    dbg!(&result);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert!(!configs.is_empty());

    // Test update_portfolio_config - Success!
    let updated_portfolio_config = portfolio_specs::get_test_update_portfolio_config();
    let result = cmdb_client
        .update_portfolio_config(updated_portfolio_config.clone())
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    let result = cmdb_client
        .read_portfolio_config_by_id(portfolio_config.portfolio_id() as u16)
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    let db_updated_portfolio_config = result.unwrap().unwrap();
    assert_eq!(db_updated_portfolio_config, updated_portfolio_config);

    // Test update_portfolio_config - Error: portfolio config does not exist
    let error_portfolio_config = portfolio_specs::get_test_update_error_portfolio_config();
    let result = cmdb_client
        .update_portfolio_config(error_portfolio_config)
        .await;
    dbg!(&result);
    assert!(result.is_err());

    // Test delete_portfolio_config - Success!
    let result = cmdb_client
        .delete_portfolio_config(portfolio_config.portfolio_id() as u16)
        .await;
    dbg!(&result);
    assert!(result.is_ok());
    let is_deleted = result.unwrap();
    assert!(is_deleted);

    // Test delete_portfolio_config - Error: portfolio config does not exist
    let result = cmdb_client.delete_portfolio_config(787).await;
    dbg!(&result);
    assert!(result.is_ok());
    let is_deleted = result.unwrap();
    assert!(!is_deleted);

    // Stop and remove container
    let delete_container = true;
    let result = docker_util.stop_container(&pg_container_id, delete_container);
    dbg!(&result);
    assert!(result.is_ok());
}
