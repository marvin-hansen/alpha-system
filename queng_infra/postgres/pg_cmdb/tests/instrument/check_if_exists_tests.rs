use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_check_if_instrument_code_exists() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    // Create a new connection
    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_test_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}
