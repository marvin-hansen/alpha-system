use common_database::prelude::PostgresDBSchema;
use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};
use diesel::Connection;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::{get_test_portfolio, postgres_schema_setup};
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_update_portfolio() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

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
