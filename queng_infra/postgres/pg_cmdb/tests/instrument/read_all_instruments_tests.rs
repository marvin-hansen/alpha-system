use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;

use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_read_all_instruments() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_test_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_instruments = result.unwrap();
    assert!(all_instruments.len() > 0);
}
