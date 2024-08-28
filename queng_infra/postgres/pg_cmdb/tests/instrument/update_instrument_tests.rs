use common_database::prelude::PostgresDBSchema::CMDB;
use common_exchange::prelude::Instrument as CommonInstrument;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;

pub const DB_URL: &str = database_utils::DB_TEST_URL;

#[tokio::test]
async fn test_update_instrument() {
    let result = database_utils::postgres_test_setup(CMDB, DB_URL).await;
    //dbg!(&result);
    assert!(result.is_ok());

    let mut connection = database_utils::postgres_connection(DB_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = database_utils::get_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let update = CommonInstrument::new(
        "test_code".to_string(),
        "new_test_class".to_string(),
        "test_exchange_code".to_string(),
        "test_exchange_pair_code".to_string(),
        "test_base_asset".to_string(),
        "test_quote_asset".to_string(),
        Some("test".to_string()),
    );

    let result = Instrument::update(conn, "test_code".to_string(), &update);
    // dbg!(&result);
    assert!(result.is_ok());
}
