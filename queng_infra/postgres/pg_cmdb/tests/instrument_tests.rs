use common_exchange::prelude::Instrument as CommonInstrument;
use diesel::Connection;
use env_utils::EnvUtil;
use pg_cmdb::model::instrument::Instrument;
use postgres_test_utils::prelude::get_test_instrument;
use postgres_test_utils::{get_or_wait_for_postgres_connection, DB_TEST_URL};

//
// Somehow tests seem to be executed or sorted in alphabetical order,
// so make sure that the setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Start or reuse a test postgres container
    let result = env.setup_container_postgres_db().await;
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_check_if_instrument_code_exists() {
    // Create a new connection
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let instrument = get_test_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_count_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let instrument = get_test_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_create_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

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

#[tokio::test]
async fn test_delete_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let instrument = get_test_instrument();
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

#[tokio::test]
async fn test_read_all_instruments() {
    // Create a new connection
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

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

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

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

#[tokio::test]
async fn test_update_instrument() {
    // Create a new connection
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let instrument = get_test_instrument();
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
