use common_config::prelude::ServiceID;
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

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
