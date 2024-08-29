use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::{get_test_portfolio, postgres_schema_setup};
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_read_portfolio() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    assert!(result.is_ok());

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
}
