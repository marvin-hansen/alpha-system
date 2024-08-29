use common_config::prelude::ServiceID;
use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_service_delete() {
    postgres_schema_setup(PostgresDBSchema::SMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
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
