use common_config::prelude::{ServiceConfig, ServiceID};
use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_smdb_setup() {
    let result = postgres_schema_setup(PostgresDBSchema::PostgresDBSchemaSMDB, DB_TEST_URL).await;
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let service = result.unwrap();

    let id = ServiceID::SMDB;
    let name = "name".to_string();
    let version = 1;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = Vec::from([
        common_config::prelude::Endpoint::default(),
        common_config::prelude::Endpoint::default(),
    ]);

    assert_eq!(service.svc_id(), &id);
    assert_eq!(service.name(), &name);
    assert_eq!(service.version(), version);
    assert_eq!(service.online(), online);
    assert_eq!(service.description(), &description);
    assert_eq!(service.health_check_uri(), &health_check_uri);
    assert_eq!(service.base_uri(), &base_uri);
    assert_eq!(service.dependencies(), &dependencies);
    assert_eq!(service.endpoints(), &endpoints);
}

#[tokio::test]
async fn test_check_if_service_id_exists() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::check_if_service_id_exists(conn, ServiceID::SMDB);
    // dbg!(&result);
    assert!(result.is_ok());

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::check_if_service_id_exists(conn, ServiceID::SMDB);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_check_if_service_id_online() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::check_if_service_id_online(conn, ServiceID::SMDB);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_check_if_service_id_online_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::check_if_service_id_online(conn, ServiceID::SMDB);
    // dbg!(&result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_count_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_service_read_all() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let services = result.unwrap();
    assert_eq!(services.len(), 0);

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let services = result.unwrap();
    assert_eq!(services.len(), 1);
}
#[tokio::test]
async fn test_service_read() {
    let service_id = ServiceID::SMDB;

    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::read(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());

    let service = result.unwrap();
    assert_eq!(service.svc_id(), &service_id);
    assert_eq!(service.name(), "name");
    assert_eq!(service.version(), 1);
    assert_eq!(service.online(), true);
    assert_eq!(service.description(), "description");
    assert_eq!(service.health_check_uri(), "health_check_uri");
    assert_eq!(service.base_uri(), "base_uri");
    assert_eq!(service.dependencies(), &vec![ServiceID::DBGW]);
    assert_eq!(
        service.endpoints(),
        &vec![
            common_config::prelude::Endpoint::default(),
            common_config::prelude::Endpoint::default(),
        ]
    );
}

#[tokio::test]
async fn test_service_read_error() {
    let service_id = ServiceID::SMDB;

    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::read(conn, service_id);
    // dbg!(&result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_update() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let param_service_id = ServiceID::SMDB;
    let result = service::Service::check_if_service_id_exists(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let id = ServiceID::SMDB;
    let name = "new_name".to_string();
    let version = 2;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = Vec::from([
        common_config::prelude::Endpoint::default(),
        common_config::prelude::Endpoint::default(),
    ]);

    let update = ServiceConfig::new(
        id.clone(),
        name.clone(),
        version.clone(),
        online.clone(),
        description.clone(),
        health_check_uri.clone(),
        base_uri.clone(),
        dependencies.clone(),
        endpoints.clone(),
    );
    let result = service::Service::update(conn, &param_service_id, &update);
    // dbg!(&result);
    assert!(result.is_ok());

    let service = result.unwrap();
    assert_eq!(service.name(), "new_name");
    assert_eq!(service.version(), 2);
    assert_eq!(service.online(), true);
    assert_eq!(service.description(), "description");
    assert_eq!(service.health_check_uri(), "health_check_uri");
    assert_eq!(service.base_uri(), "base_uri");
    assert_eq!(service.dependencies(), &vec![ServiceID::DBGW]);
    assert_eq!(
        service.endpoints(),
        &vec![
            common_config::prelude::Endpoint::default(),
            common_config::prelude::Endpoint::default(),
        ]
    );
}

#[tokio::test]
async fn test_service_update_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    // Double check the service isn't in the database
    let param_service_id = ServiceID::SMDB;
    let result = service::Service::check_if_service_id_exists(conn, param_service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());

    let id = ServiceID::SMDB;
    let name = "new_name".to_string();
    let version = 2;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = Vec::from([
        common_config::prelude::Endpoint::default(),
        common_config::prelude::Endpoint::default(),
    ]);

    let update = ServiceConfig::new(
        id.clone(),
        name.clone(),
        version.clone(),
        online.clone(),
        description.clone(),
        health_check_uri.clone(),
        base_uri.clone(),
        dependencies.clone(),
        endpoints.clone(),
    );
    let result = service::Service::update(conn, &param_service_id, &update);
    // dbg!(&result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_delete() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let service_config = get_test_service_config();
    let result = service::Service::create(conn, &service_config);
    // dbg!(&result);
    assert!(result.is_ok());

    let service_id = ServiceID::SMDB;

    let result = service::Service::check_if_service_id_exists(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = service::Service::delete(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = service::Service::read(conn, service_id);
    // dbg!(&result);
    assert!(result.is_err());

    let result = service::Service::check_if_service_id_exists(conn, service_id);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());

    let result = service::Service::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
