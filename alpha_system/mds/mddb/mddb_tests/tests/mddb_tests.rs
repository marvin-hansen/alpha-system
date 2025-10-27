/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use config_manager::CfgManager;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use mddb_client::MDDBClient;
use service_import::ServiceImportManager;
use service_utils::{ServiceStartConfig, ServiceUtil};

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

pub const ROOT_PATH: &str = "alpha_system/mds/mddb/mddb_tests/tests";

pub const BINARIES: [&str; 3] = ["dbgw", "smdb", "mddb"];

#[tokio::test]
async fn test_mddb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.get_or_start_container(&pg_container_config);
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();
    println!("✅ Postgres container ID: {pg_container_id} started");

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

    dbg!("Start MDDB service - depends on SMDB");
    let (host, port) = config_manager
        .get_mddb_host_port()
        .await
        .expect("Failed to get host and port for MDDB");

    let mddb_start_config = get_service_start_config("mddb", host, port);
    let result = svc_util.start_service_from_config(mddb_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ MDDB service started");

    dbg!("Configure MDDB client");
    let (host, port) = config_manager
        .get_mddb_host_port()
        .await
        .expect("Failed to get MDDB host");
    dbg!(&host);
    dbg!(&port);

    // Construct MDDB client
    let _ = MDDBClient::new(host, port)
        .await
        .expect("Failed to create MDDB client");
    dbg!("✅ MDDB client started");

    // Stop and remove container
    let delete_container = true;
    let result = docker_util.stop_container(&pg_container_id, delete_container);
    dbg!(&result);
    assert!(result.is_ok());
}
