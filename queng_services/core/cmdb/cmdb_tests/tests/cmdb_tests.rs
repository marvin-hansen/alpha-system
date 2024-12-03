use cmdb_client::CmdbClient;
use common_config::ServiceID;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use service_import::ServiceImportManager;
use service_utils::{ServiceUtil, ServiceWaitStrategy};
use std::time::Duration;

async fn get_service_wait_strategy(host: String, port: u16) -> ServiceWaitStrategy {
    let url = format!("http://{host}:{port}");
    ServiceWaitStrategy::GrpcHealthCheck(url, Duration::from_secs(10))
}

#[tokio::test]
async fn test_cmdb() {
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

    // Start CMDDB service - depends on SMDB and DBGW
    let service_id = ServiceID::CMDB;
    let (host, port) = config_manager
        .get_cmdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");
    let wait_strategy = get_service_wait_strategy(host, port).await;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Configure CMDB client
    let (cmdb_host, cmdb_port) = config_manager
        .get_cmdb_host_port()
        .await
        .expect("Failed to get SMDB host");
    dbg!(&cmdb_host);
    dbg!(&cmdb_port);

    // Construct CMDB client
    let cmdb_client = CmdbClient::new(cmdb_host, cmdb_port).await;

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
    let result = docker_util.stop_container(&pg_container_id);
    dbg!(&result);
    assert!(result.is_ok());
}
