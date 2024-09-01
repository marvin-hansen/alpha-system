use common_config::prelude::ServiceID;
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
async fn test_check_if_service_id_exists() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::check_if_service_id_exists(conn, ServiceID::SMDB);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());

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

    let result = service::Service::check_if_service_id_online(conn, ServiceID::SMDB);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());

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
async fn test_create_instrument() {
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
