use diesel::Connection;
use env_utils::EnvUtil;
use pg_metadb::prelude::{Exchange, Instrument, InstrumentsExchanges};
use postgres_test_utils::prelude::{get_test_meta_exchange, get_test_meta_instrument};
use postgres_test_utils::{get_or_wait_for_postgres_connection, DB_TEST_URL};

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Start or reuse a test postgres container
    let result = env.setup_container_postgres_db().await;
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_migration() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_metadb::run_metadb_migration(conn);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_instruments_exchanges() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_metadb::run_metadb_migration(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    // Insert exchange
    let test_data = get_test_meta_exchange();
    let result = Exchange::create(conn, test_data);
    assert!(result.is_ok());

    // Insert instrument
    let instrument = get_test_meta_instrument();
    let result = Instrument::create(conn, instrument);
    assert!(result.is_ok());

    // Insert the relation between exchange and instrument
    let instrument_id = String::from("BTC-USD");
    let exchange_id = String::from("test_exchange_code");
    let result = InstrumentsExchanges::create(conn, instrument_id, exchange_id);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_instruments_exchanges_err() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_metadb::run_metadb_migration(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    // The
    let instrument_id = String::from("InvalidTestInstrument");
    let exchange_id = String::from("InvalidTestExchange");
    let result = InstrumentsExchanges::create(conn, instrument_id, exchange_id);
    // dbg!(&result);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_check_if_instruments_exchanges_exists() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_metadb::run_metadb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_data = get_test_meta_exchange();
    let result = Exchange::create(conn, test_data);
    assert!(result.is_ok());

    // Insert instrument
    let instrument = get_test_meta_instrument();
    let result = Instrument::create(conn, instrument);
    assert!(result.is_ok());

    // Insert the relation between exchange and instrument
    let instrument_id = String::from("BTC-USD");
    let exchange_id = String::from("test_exchange_code");
    let result = InstrumentsExchanges::create(conn, instrument_id.clone(), exchange_id.clone());
    // dbg!(&result);
    assert!(result.is_ok());

    // Check if InstrumentsExchanges exists
    let exists = InstrumentsExchanges::check_if_exists(conn, instrument_id, exchange_id)
        .expect("Failed to check if InstrumentsExchanges exists");
    assert!(exists);
}

#[tokio::test]
async fn test_delete_instruments_exchanges() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_metadb::run_metadb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_data = get_test_meta_exchange();
    let result = Exchange::create(conn, test_data);
    assert!(result.is_ok());

    // Insert instrument
    let instrument = get_test_meta_instrument();
    let result = Instrument::create(conn, instrument);
    assert!(result.is_ok());

    // Insert the relation between exchange and instrument
    let instrument_id = String::from("BTC-USD");
    let exchange_id = String::from("test_exchange_code");
    let result = InstrumentsExchanges::create(conn, instrument_id.clone(), exchange_id.clone());
    // dbg!(&result);
    assert!(result.is_ok());

    // Check if InstrumentsExchanges exists
    let exists =
        InstrumentsExchanges::check_if_exists(conn, instrument_id.clone(), exchange_id.clone())
            .expect("Failed to check if InstrumentsExchanges exists");
    assert!(exists);

    // Delete InstrumentsExchanges
    let result = InstrumentsExchanges::delete(conn, instrument_id, exchange_id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}
