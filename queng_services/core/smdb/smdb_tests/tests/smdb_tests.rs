use common_config::ServiceID;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use service_import::ServiceImportManager;
use service_utils::{ServiceUtil, ServiceWaitStrategy};
use smdb_client::SMDBClient;
use std::time::Duration;

async fn get_service_wait_strategy(host: String, port: u16) -> ServiceWaitStrategy {
    let url = format!("http://{host}:{port}");
    ServiceWaitStrategy::GrpcHealthCheck(url, Duration::from_secs(10))
}

#[tokio::test]
async fn test_smdb() {
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

    // Configure SMDB client
    let (smdb_host, smdb_port) = config_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get SMDB host");
    dbg!(&smdb_host);
    dbg!(&smdb_port);
    let smdb_client = SMDBClient::new(smdb_host, smdb_port).await;

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
    let result = docker_util.stop_container(&pg_container_id);
    dbg!(&result);
    assert!(result.is_ok());
}
