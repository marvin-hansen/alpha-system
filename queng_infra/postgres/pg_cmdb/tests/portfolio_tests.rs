use common_exchange::prelude::AccountType;
use common_exchange::prelude::PortfolioConfig as CommonPortfolioConfig;
use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_migrations::prelude::{get_or_wait_for_postgres_connection, DB_TEST_URL};

fn get_test_portfolio() -> CommonPortfolioConfig {
    CommonPortfolioConfig::new(
        1,
        "Test Portfolio".to_string(),
        AccountType::Spot,
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
    )
}

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
async fn test_create_portfolio() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);

    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_cmdb::run_cmdb_db_migration(conn);
    // dbg!(&result);
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
    // dbg!(&result);
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
