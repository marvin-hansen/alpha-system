use common_metadata::prelude::{InstrumentMetadata, MetaExchange, MetaInstrument};
use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use pg_mddb::prelude::{Exchange, Instrument, InstrumentsExchanges};
use postgres_migrations::prelude::{get_or_wait_for_postgres_connection, DB_TEST_URL};

fn get_test_meta_exchange() -> MetaExchange {
    MetaExchange {
        code: "test_exchange_code".to_string(),
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}

fn get_test_meta_instrument() -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: Some("2021-12-31T23:59:59Z".to_string()),
        exchange_code: "test_exchange_code".to_string(),
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

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config); // dbg!(&result);
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

    let result = pg_mddb::run_mddb_migration(conn);
    dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let instrument = get_test_meta_instrument();
    let result = Instrument::create_instrument(conn, instrument);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_instrument_collection_success() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let meta_instruments = vec![get_test_meta_instrument()];
    let result = Instrument::create_instrument_collection(conn, &meta_instruments);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_create_instrument_collection_empty() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let meta_instruments: Vec<MetaInstrument> = vec![];
    let result = Instrument::create_instrument_collection(conn, &meta_instruments);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_instrument_collection_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    // All test data have the same instrument code (primary key)
    // hence triggering a unique constraint violation error
    let meta_instruments = vec![
        get_test_meta_instrument(),
        get_test_meta_instrument(),
        get_test_meta_instrument(),
    ];
    let result = Instrument::create_instrument_collection(conn, &meta_instruments);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_count_instruments_with_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let test_data = get_test_meta_instrument();
    let result = Instrument::create_instrument(conn, test_data);
    assert!(result.is_ok());

    let result = Instrument::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_count_instruments_no_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Instrument::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[tokio::test]
async fn test_check_if_instrument_id_exists_returns_true() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let instrument = get_test_meta_instrument();
    let instrument_id = instrument.primary_key();
    let result = Instrument::create_instrument(conn, instrument);
    assert!(result.is_ok());

    let exists = Instrument::check_if_instrument_id_exists(conn, &instrument_id)
        .expect("Failed to check if instrument ID exists");
    assert!(exists);
}

#[tokio::test]
async fn test_check_if_instrument_id_exists_returns_false() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let instrument_id = "non_existent_id";
    let exists = Instrument::check_if_instrument_id_exists(conn, instrument_id)
        .expect("Failed to check if instrument ID exists");
    assert!(!exists);
}

#[tokio::test]
async fn test_read_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let instrument = get_test_meta_instrument();
    let instrument_id = instrument.primary_key();
    let result = Instrument::create_instrument(conn, instrument);
    assert!(result.is_ok());

    let result = Instrument::read(conn, &instrument_id);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_read_instrument_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let instrument_id = "Non-Existent".to_string();

    let result = Instrument::read(conn, &instrument_id);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_read_all_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let instrument = get_test_meta_instrument();
    let result = Instrument::create_instrument(conn, instrument);
    assert!(result.is_ok());

    let result = Instrument::read_all(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn test_read_all_instrument_no_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Instrument::read_all(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[tokio::test]
async fn test_update_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    let instrument = get_test_meta_instrument();
    let instrument_id = instrument.primary_key();
    let result = Instrument::create_instrument(conn, instrument);
    assert!(result.is_ok());

    let update_data = get_test_meta_instrument();
    let result = Instrument::update(conn, &instrument_id, update_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1)
}

#[tokio::test]
async fn test_update_instrument_non_existent() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let instrument_id = String::from("Non-Existent");
    let update_data = get_test_meta_instrument();
    let result = Instrument::update(conn, &instrument_id, update_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0)
}

#[tokio::test]
async fn test_delete_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Insert exchange
    let test_exchange = get_test_meta_exchange();
    let exchange_id = test_exchange.code.to_string();
    let result = Exchange::create_exchange(conn, test_exchange);
    assert!(result.is_ok());

    // Insert instrument
    let instrument = get_test_meta_instrument();
    let instrument_id = instrument.primary_key();
    Instrument::create_instrument(conn, instrument).expect("Failed to create instrument");

    // Check if InstrumentsExchanges exists
    let exists =
        InstrumentsExchanges::check_if_exists(conn, instrument_id.clone(), exchange_id.clone())
            .expect("Failed to check if InstrumentsExchanges exists");
    assert!(exists);

    println!("Exchange ID: {}", &exchange_id);
    println!("Instrument ID: {}", &instrument_id);

    let result = Instrument::delete(conn, instrument_id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1)
}

#[tokio::test]
async fn test_delete_instrument_non_existent() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let instrument_id = String::from("Non-Existent");
    let result = Instrument::delete(conn, instrument_id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0)
}
