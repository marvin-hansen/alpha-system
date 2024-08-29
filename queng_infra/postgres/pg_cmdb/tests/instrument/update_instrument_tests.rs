use common_database::prelude::PostgresDBSchema;
use common_exchange::prelude::Instrument as CommonInstrument;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_update_instrument() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();
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
