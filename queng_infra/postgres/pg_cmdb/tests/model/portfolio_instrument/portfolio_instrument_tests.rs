use common_exchange::AccountType::Spot;
use common_exchange::Instrument as CommonInstrument;
use common_exchange::PortfolioConfig as CommonPortfolioConfig;
use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::DockerUtil;
use pg_cmdb::model::instrument::Instrument;
use pg_cmdb::model::portfolio::Portfolio;
use pg_cmdb::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use postgres_migrations::{get_or_wait_for_postgres_connection, DB_TEST_URL};

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get EnvUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config); // dbg!(&result);
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

    let result =
        PortfolioInstrument::check_if_exists(conn, portfolio_id as i32, instrument_id.to_string());
    //dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = PortfolioInstrument::read_instruments_for_portfolio(conn, portfolio_id as i32);
    //dbg!(&result);
    assert!(result.is_ok());

    let portfolio_instruments = result.unwrap();
    assert!(!portfolio_instruments.is_empty());

    let result = PortfolioInstrument::delete(conn, portfolio_id as i32, instrument_id.to_string());
    //dbg!(&result);
    assert!(result.is_ok());

    let result =
        PortfolioInstrument::check_if_exists(conn, portfolio_id as i32, instrument_id.to_string());
    //dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
