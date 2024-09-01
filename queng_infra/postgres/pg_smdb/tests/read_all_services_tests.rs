use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

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
