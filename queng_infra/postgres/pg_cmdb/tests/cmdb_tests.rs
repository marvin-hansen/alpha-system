use common_exchange::prelude::AccountType::Spot;
use common_exchange::prelude::PortfolioConfig as CommonPortfolioConfig;
use common_exchange::prelude::{AccountType, Instrument as CommonInstrument};
use diesel::Connection;
use pg_cmdb::model::instrument::Instrument;
use pg_cmdb::model::portfolio::Portfolio;
use pg_cmdb::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use postgres_test_utils::prelude::{get_test_instrument, get_test_portfolio};
use postgres_test_utils::{get_or_wait_for_postgres_connection, DB_TEST_URL};

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

#[tokio::test]
async fn test_portfolio_instrument() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    //
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let portfolio_id = 42;
    let create_portfolio = CommonPortfolioConfig::new(
        portfolio_id,
        "Test Portfolio".to_string(),
        Spot,
        "12345".to_string(),
        "USD".to_string(),
        1000.0,
        500.0,
        20.0,
        Vec::new(),
        30.0,
        10.0,
        500.0,
        1000.0,
        50.0,
        100.0,
    );

    //
    let result = Portfolio::create(conn, &create_portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let instrument_id = "test42";

    let instrument = CommonInstrument::new(
        instrument_id.to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        Some("test".to_string()),
    );

    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    // Create Portfolio Instrument using the Portfolio ID and Instrument ID
    let create_portfolio_instrument =
        CreatePortfolioInstrument::new(portfolio_id as i32, instrument_id.to_string());

    // Insert Portfolio Instrument
    let result = PortfolioInstrument::create(conn, &create_portfolio_instrument);
    //dbg!(&result);
    assert!(result.is_ok());

    let portfolio_instrument = result.unwrap();

    assert_eq!(portfolio_instrument.portfolio_id, 42);
    assert_eq!(portfolio_instrument.instrument_id, "test42");

    let result = PortfolioInstrument::read_instruments_for_portfolio(conn, portfolio_id as i32);
    //dbg!(&result);
    assert!(result.is_ok());

    let portfolio_instruments = result.unwrap();
    assert!(portfolio_instruments.len() > 0);

    let result = PortfolioInstrument::delete(conn, portfolio_id as i32, instrument_id.to_string());
    //dbg!(&result);
    assert!(result.is_ok());

    let result = PortfolioInstrument::read_instruments_for_portfolio(conn, 1);
    //dbg!(&result);
    assert!(result.is_err());

    let result = Instrument::delete(conn, instrument_id.to_string());
    //dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::delete(conn, portfolio_id as i32);
    //dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_check_if_portfolio_id_exists() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_count_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let result = Portfolio::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_read_all_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let result = Portfolio::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_portfolios = result.unwrap();
    assert_eq!(all_portfolios.len(), 0);

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    let all_portfolios = result.unwrap();
    assert_eq!(all_portfolios.len(), 1);
}

#[tokio::test]
async fn test_read_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::read(conn, 1);
    // dbg!(&result);
    assert!(result.is_ok());

    let portfolio = result.unwrap();
    assert_eq!(portfolio.portfolio_id(), 1);
    assert_eq!(portfolio.portfolio_description(), "Test Portfolio");
    assert_eq!(portfolio.portfolio_account_type(), AccountType::Spot);
    assert_eq!(portfolio.portfolio_account_id(), "12345");
    assert_eq!(portfolio.portfolio_currency(), "USD");
    assert_eq!(portfolio.portfolio_cash(), 1000.0);
    assert_eq!(portfolio.portfolio_margin(), 500.0);
    assert_eq!(portfolio.portfolio_max_drawdown(), 20.0);
    assert_eq!(portfolio.instrument_max_allocation(), 30.0);
    assert_eq!(portfolio.instrument_max_drawdown(), 10.0);
    assert_eq!(portfolio.portfolio_free_margin(), 500.0);
    assert_eq!(portfolio.portfolio_free_cash(), 1000.0);
    assert_eq!(portfolio.portfolio_free_margin_percent(), 50.0);
    assert_eq!(portfolio.portfolio_free_cash_percent(), 100.0);
}

#[tokio::test]
async fn test_update_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let update = CommonPortfolioConfig::new(
        1,
        "Updated Portfolio".to_string(),
        AccountType::Spot,
        "67890".to_string(),
        "EUR".to_string(),
        1000.0,
        500.0,
        20.0,
        Vec::new(),
        3.0,
        10.0,
        1000.0,
        2000.0,
        50.0,
        100.0,
    );

    let result = Portfolio::update(conn, 1, &update);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_portfolio_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let portfolio_id = 23;

    let update = CommonPortfolioConfig::new(
        portfolio_id,
        "Updated Portfolio".to_string(),
        AccountType::Spot,
        "67890".to_string(),
        "EUR".to_string(),
        1000.0,
        500.0,
        20.0,
        Vec::new(),
        3.0,
        10.0,
        1000.0,
        2000.0,
        50.0,
        100.0,
    );

    let result = Portfolio::update(conn, portfolio_id as i32, &update);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    assert!(result.is_ok());

    let result = Portfolio::delete(conn, 1);
    dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
