use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_count_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let instrument = get_test_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}
