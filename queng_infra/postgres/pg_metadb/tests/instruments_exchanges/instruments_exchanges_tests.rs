use common_metadata::prelude::{InstrumentMetadata, MetaExchange, MetaInstrument};
use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use pg_metadb::prelude::{Exchange, Instrument, InstrumentsExchanges};
use postgres_migrations::prelude::{get_or_wait_for_postgres_connection, DB_TEST_URL};

fn get_test_meta_instrument() -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: Some("2021-12-31T23:59:59Z".to_string()),
        exchange_code: "XKRX".to_string(),
        exchange_pair_code: "BTCUSD".to_string(),
        base_asset: "BTC".to_string(),
        quote_asset: "USD".to_string(),
        kaiko_legacy_symbol: "BTCUSD".to_string(),
        code: "BTC-USD".to_string(),
        class: "currency".to_string(),
        metadata: Some(metadata),
        trade_start_timestamp: Some(1609459200),
        trade_end_timestamp: Some(1640995199),
        trade_compressed_size: 1024,
        trade_count: 10000,
    }
}

fn get_test_meta_exchange() -> MetaExchange {
    MetaExchange {
        code: "test_exchange_code".to_string(),
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

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
