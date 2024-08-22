use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use pg_cmdb::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use pg_cmdb::run_cmdb_db_migration;

fn postgres_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = "postgres://postgres:postgres@localhost/postgres";

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn test_db_migration(conn: &mut pg_cmdb::Connection) {
    let res = run_cmdb_db_migration(conn);
    //dbg!(&result);
    assert!(res.is_ok());
}

#[test]
fn test_portfolio() {
    let pool = postgres_connection_pool();
    let mut conn = &mut pool.get().unwrap();

    println!("Test DB migration!");
    test_db_migration(conn);

    let portfolio = CreatePortfolio::new(
        1,
        "Test Portfolio".to_string(),
        1,
        "12345".to_string(),
        "USD".to_string(),
        1000.0,
        500.0,
        20.0,
        30.0,
        10.0,
        500.0,
        1000.0,
        50.0,
        100.0,
    );

    let result = Portfolio::create(&mut conn, &portfolio);
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

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = Portfolio::read(&mut conn, 1);
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

    let result = Portfolio::read_all(&mut conn);
    assert!(result.is_ok());

    let all_portfolios = result.unwrap();
    assert!(all_portfolios.len() > 0);

    let update = UpdatePortfolio::new(
        Some("Updated Portfolio".to_string()),
        Some(2),
        Some("67890".to_string()),
        Some("EUR".to_string()),
        Some(2000.0),
        Some(1000.0),
        Some(30.0),
        Some(40.0),
        Some(20.0),
        Some(1000.0),
        Some(2000.0),
        Some(50.0),
        Some(100.0),
    );

    let result = Portfolio::update(&mut conn, 1, &update);
    assert!(result.is_ok());

    let portfolio = result.unwrap();

    assert_eq!(portfolio.portfolio_description, "Updated Portfolio");
    assert_eq!(portfolio.portfolio_account_type, 2);
    assert_eq!(portfolio.portfolio_account_id, "67890");
    assert_eq!(portfolio.portfolio_currency, "EUR");
    assert_eq!(portfolio.portfolio_cash, 2000.0);
    assert_eq!(portfolio.portfolio_margin, 1000.0);
    assert_eq!(portfolio.portfolio_max_drawdown, 30.0);
    assert_eq!(portfolio.instrument_max_allocation, 40.0);
    assert_eq!(portfolio.instrument_max_drawdown, 20.0);
    assert_eq!(portfolio.portfolio_free_margin, 1000.0);
    assert_eq!(portfolio.portfolio_free_cash, 2000.0);
    assert_eq!(portfolio.portfolio_free_margin_percent, 50.0);
    assert_eq!(portfolio.portfolio_free_cash_percent, 100.0);

    let result = Portfolio::read(&mut conn, 1);
    assert!(result.is_ok());

    let result = Portfolio::delete(&mut conn, 1);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
