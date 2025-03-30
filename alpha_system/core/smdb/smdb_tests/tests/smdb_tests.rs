/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_config::ServiceID;
use config_manager::CfgManager;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use service_import::ServiceImportManager;
use service_utils::*;
use smdb_client::*;

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

pub const ROOT_PATH: &str = "alpha_system/core/smdb/smdb_tests/tests";
pub const BINARIES: [&str; 2] = ["dbgw", "smdb"];

#[tokio::test]
async fn test_smdb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.setup_container(&pg_container_config);
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

    dbg!("Configure SMDB client");
    let (smdb_host, smdb_port) = config_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get SMDB host");
    dbg!(&smdb_host);
    dbg!(&smdb_port);
    let smdb_client = SMDBClient::new(smdb_host, smdb_port).await;
    dbg!("✅ SMDB client started");

    // Test: check_if_service_id_exists - Service exists
    let res = smdb_client
        .check_if_service_id_exists(ServiceID::DBGW)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(exists);

    let res = smdb_client
        .check_if_service_id_exists(ServiceID::SMDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(exists);

    let res = smdb_client
        .check_if_service_id_exists(ServiceID::CMDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(exists);

    let res = smdb_client
        .check_if_service_id_exists(ServiceID::MDDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(exists);

    // Test: check_if_service_id_exists - Service NOT exists
    let res = smdb_client
        .check_if_service_id_exists(ServiceID::Default)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(!exists);

    // Test: check_if_services_exists - All Services exists
    let services = vec![
        ServiceID::DBGW,
        ServiceID::SMDB,
        ServiceID::CMDB,
        ServiceID::MDDB,
    ];
    let res = smdb_client.check_if_services_exists(services).await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(exists);

    // Test: check_if_services_exists - All Services NOT exists
    let services = vec![
        ServiceID::DBGW,
        ServiceID::SMDB,
        ServiceID::CMDB,
        ServiceID::Default,
    ];
    let res = smdb_client.check_if_services_exists(services).await;
    dbg!(&res);
    assert!(res.is_ok());
    let exists = res.unwrap();
    assert!(!exists);

    // Test: check_if_service_id_online - Service is online
    let res = smdb_client
        .check_if_service_id_online(ServiceID::DBGW)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    let res = smdb_client
        .check_if_service_id_online(ServiceID::SMDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Test: check_if_service_id_online - Service NOT online
    let res = smdb_client
        .check_if_service_id_online(ServiceID::CMDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    let res = smdb_client
        .check_if_service_id_online(ServiceID::MDDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    // Test: check_if_service_id_online - Service DOES NOT EXIST
    let res = smdb_client
        .check_if_service_id_online(ServiceID::Default)
        .await;
    dbg!(&res);
    assert!(res.is_err());

    // Test: check_if_services_online - All Services online
    let services = vec![ServiceID::DBGW, ServiceID::SMDB];
    let res = smdb_client.check_if_services_online(services).await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Test: check_if_services_online - All Services NOT online
    let services = vec![ServiceID::CMDB, ServiceID::MDDB];
    let res = smdb_client.check_if_services_online(services).await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    // Test: check_if_services_online -  Service DOES NOT EXIST
    let services = vec![ServiceID::Default];
    let res = smdb_client.check_if_services_online(services).await;
    dbg!(&res);
    assert!(res.is_err());

    // Test: set_service_online - Service exists and is OFFLINE. Set it ONLINE.
    // Not online
    let res = smdb_client
        .check_if_service_id_online(ServiceID::MDDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    // Set online
    let res = smdb_client.set_service_online(ServiceID::MDDB).await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Service is online
    let res = smdb_client
        .check_if_service_id_online(ServiceID::MDDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Test: set_service_online - Service exists and is already ONLINE.
    // Service is online
    let res = smdb_client
        .check_if_service_id_online(ServiceID::DBGW)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Set online
    let res = smdb_client.set_service_online(ServiceID::DBGW).await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Test: set_service_online - Service DOES NOT EXIST
    let res = smdb_client.set_service_online(ServiceID::Default).await;
    dbg!(&res);
    assert!(res.is_err());

    // Test: set_service_offline - Service exists and is ONLINE. Set it OFFLINE.

    // Service is online
    let res = smdb_client
        .check_if_service_id_online(ServiceID::MDDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(online);

    // Set offline
    let res = smdb_client.set_service_offline(ServiceID::MDDB).await;
    dbg!(&res);
    assert!(res.is_ok());
    let offline = res.unwrap();
    assert!(offline);

    // Service is offline
    let res = smdb_client
        .check_if_service_id_online(ServiceID::MDDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    // Test: set_service_offline - Service exists and is already OFFLINE.
    // Service is offline
    let res = smdb_client
        .check_if_service_id_online(ServiceID::CMDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    // Set offline
    let res = smdb_client.set_service_offline(ServiceID::CMDB).await;
    dbg!(&res);
    assert!(res.is_ok());
    let offline = res.unwrap();
    assert!(offline);

    // Service is offline
    let res = smdb_client
        .check_if_service_id_online(ServiceID::CMDB)
        .await;
    dbg!(&res);
    assert!(res.is_ok());
    let online = res.unwrap();
    assert!(!online);

    // Test: set_service_offline - Service DOES NOT EXIST
    let res = smdb_client.set_service_offline(ServiceID::Default).await;
    dbg!(&res);
    assert!(res.is_err());

    // Set offline
    let res = smdb_client
        .check_if_service_id_exists(ServiceID::Default)
        .await;
    dbg!(&res);

    // Drop SMDBClient as it is not needed anymore
    drop(smdb_client);

    // Stop and remove container
    let delete_container = true;
    let result = docker_util.stop_container(&pg_container_id, delete_container);
    dbg!(&result);
    assert!(result.is_ok());
}
