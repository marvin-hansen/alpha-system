use common_config::prelude::ServiceID;
use diesel::Connection;
use pg_smdb::model::service;
use postgres_test_utils::prelude::*;

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
