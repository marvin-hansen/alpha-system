use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};
use diesel::Connection;

use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_portfolio() {
    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let portfolio = CommonPortfolioConfig::new(
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
    );

    let result = Portfolio::create(conn, &portfolio);
    assert!(result.is_ok());

    let portfolio = result.unwrap();

    assert_eq!(portfolio.portfolio_id(), 1);
    assert_eq!(portfolio.portfolio_description(), "Test Portfolio");
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

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = Portfolio::read(conn, 1);
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

    let result = Portfolio::read_all(conn);
    assert!(result.is_ok());

    let all_portfolios = result.unwrap();
    assert!(all_portfolios.len() > 0);

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

    let result = Portfolio::read(conn, 1);
    assert!(result.is_ok());

    let result = Portfolio::delete(conn, 1);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
