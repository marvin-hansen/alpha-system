use common_config::prelude::ServiceID;
use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_service_read() {
    let service_id = ServiceID::SMDB;

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

    postgres_schema_setup(PostgresDBSchema::SMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = service::Service::read(conn, service_id);
    // dbg!(&result);
    assert!(result.is_err());
}
