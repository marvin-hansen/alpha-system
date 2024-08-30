use common_database::prelude::PostgresDBSchema;
use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};
use diesel::{Connection, PgConnection};
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::{get_test_portfolio, postgres_schema_setup};
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_portfolio() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    println!("Test test_create_portfolio!");
    test_create_portfolio(conn).await;

    println!("Test test_check_if_portfolio_id_exists!");
    test_check_if_portfolio_id_exists(conn).await;

    println!("Test test_count_portfolio!");
    test_count_portfolio(conn).await;

    println!("Test test_read_all_portfolio!");
    test_read_all_portfolio(conn).await;

    println!("Test test_read_portfolio!");
    test_read_portfolio(conn).await;

    println!("Test test_update_portfolio!");
    test_update_portfolio(conn).await;

    println!("Test test_update_portfolio_error!");
    test_update_portfolio_error(conn).await;

    println!("Test test_delete_portfolio!");
    test_delete_portfolio(conn).await;
}

async fn test_create_portfolio(conn: &mut PgConnection) {
    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());
}

async fn test_check_if_portfolio_id_exists(conn: &mut PgConnection) {
    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

async fn test_count_portfolio(conn: &mut PgConnection) {
    let result = Portfolio::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

async fn test_read_all_portfolio(conn: &mut PgConnection) {
    let result = Portfolio::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_portfolios = result.unwrap();
    assert_eq!(all_portfolios.len(), 1);
}

async fn test_read_portfolio(conn: &mut PgConnection) {
    let result = Portfolio::read(conn, 1);
    // dbg!(&result);
    assert!(result.is_ok());

    let portfolio = result.unwrap();
    assert_eq!(portfolio.portfolio_id, 1);
    assert_eq!(portfolio.portfolio_description, "Test Portfolio");
    assert_eq!(portfolio.portfolio_account_type, 1);
    assert_eq!(portfolio.portfolio_account_id, "12345");
    assert_eq!(portfolio.portfolio_currency, "USD");
    assert_eq!(portfolio.portfolio_cash, 1000.0);
    assert_eq!(portfolio.portfolio_margin, 500.0);
    assert_eq!(portfolio.portfolio_max_drawdown, 20.0);
    assert_eq!(portfolio.instrument_max_allocation, 30.0);
    assert_eq!(portfolio.instrument_max_drawdown, 10.0);
    assert_eq!(portfolio.portfolio_free_margin, 500.0);
    assert_eq!(portfolio.portfolio_free_cash, 1000.0);
    assert_eq!(portfolio.portfolio_free_margin_percent, 50.0);
    assert_eq!(portfolio.portfolio_free_cash_percent, 100.0);
}

async fn test_update_portfolio(conn: &mut PgConnection) {
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

    let portfolio = result.unwrap();

    assert_eq!(portfolio.portfolio_description(), "Updated Portfolio");
    assert_eq!(portfolio.portfolio_account_type(), AccountType::Spot);
    assert_eq!(portfolio.portfolio_account_id(), "67890");
    assert_eq!(portfolio.portfolio_currency(), "EUR");
    assert_eq!(portfolio.portfolio_cash(), 1000.0);
    assert_eq!(portfolio.portfolio_margin(), 500.0);
    assert_eq!(portfolio.portfolio_max_drawdown(), 20.0);
    assert_eq!(portfolio.instrument_max_allocation(), 3.0);
    assert_eq!(portfolio.instrument_max_drawdown(), 10.0);
    assert_eq!(portfolio.portfolio_free_margin(), 1000.0);
    assert_eq!(portfolio.portfolio_free_cash(), 2000.0);
    assert_eq!(portfolio.portfolio_free_margin_percent(), 50.0);
    assert_eq!(portfolio.portfolio_free_cash_percent(), 100.0);
}

async fn test_update_portfolio_error(conn: &mut PgConnection) {
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

async fn test_delete_portfolio(conn: &mut PgConnection) {
    let result = Portfolio::delete(conn, 1);
    dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
