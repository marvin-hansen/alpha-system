use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_read_instrument() {
    // Create a new connection
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_test_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let instrument = result.unwrap();
    assert_eq!(instrument.code(), "test_code");
    assert_eq!(instrument.class(), "test_class");
    assert_eq!(instrument.exchange_code(), "test_exchange_code");
    assert_eq!(instrument.exchange_pair_code(), "test_exchange_pair_code");
    assert_eq!(instrument.base_asset(), "test_base_asset");
    assert_eq!(instrument.quote_asset(), "test_quote_asset");
    assert_eq!(instrument.instrument_figi(), &Some("test".to_string()));
}
