use common_database::prelude::PostgresDBSchema::CMDB;
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;

pub const DB_URL: &str = database_utils::DB_TEST_URL;

#[tokio::test]
async fn test_delete_instrument() {
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

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = Instrument::delete(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
