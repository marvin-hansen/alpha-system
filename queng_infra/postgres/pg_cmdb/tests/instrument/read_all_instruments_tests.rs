use common_database::prelude::PostgresDBSchema::CMDB;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;

pub const DB_URL: &str = database_utils::DB_TEST_URL;

#[tokio::test]
async fn test_read_all_instruments() {
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

    let result = Instrument::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_instruments = result.unwrap();
    assert!(all_instruments.len() > 0);
}
