use common_exchange::prelude::Instrument as CommonInstrument;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use pg_cmdb::model::instrument::Instrument;
use pg_cmdb::model::portfolio::{CreatePortfolio, Portfolio};
use pg_cmdb::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use pg_cmdb::run_cmdb_db_migration;
use std::env;

fn postgres_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("POSTGRES_DATABASE_URL"))
        .expect("DATABASE_URL must be set");

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
fn test_portfolio_instrument() {
    let pool = postgres_connection_pool();
    let mut conn = &mut pool.get().unwrap();

    println!("Test DB migration!");
    test_db_migration(conn);

    let portfolio_id = 23;
    let create_portfolio = CreatePortfolio::new(
        portfolio_id,
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

    let result = Portfolio::create(&mut conn, &create_portfolio);
    assert!(result.is_ok());

    let instrument_id = "test42";

    let instrument = CommonInstrument::new(
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        Some("test".to_string()),
    );

    let result = Instrument::create(&mut conn, &instrument);
    assert!(result.is_ok());

    // Create Portfolio Instrument using the Portfolio ID and Instrument ID
    let create_portfolio_instrument =
        CreatePortfolioInstrument::new(portfolio_id, instrument_id.to_string());

    // Insert Portfolio Instrument
    let result = PortfolioInstrument::create(&mut conn, &create_portfolio_instrument);
    assert!(result.is_ok());

    let portfolio_instrument = result.unwrap();

    assert_eq!(portfolio_instrument.portfolio_id, 23);
    assert_eq!(portfolio_instrument.instrument_id, "test42");

    let result = PortfolioInstrument::read_instruments_for_portfolio(&mut conn, portfolio_id);
    assert!(result.is_ok());

    let portfolio_instruments = result.unwrap();
    assert!(portfolio_instruments.len() > 0);

    let result = PortfolioInstrument::delete(&mut conn, portfolio_id, instrument_id.to_string());
    assert!(result.is_ok());

    let result = PortfolioInstrument::read_instruments_for_portfolio(&mut conn, 1);
    assert!(result.is_err());

    let result = Instrument::delete(&mut conn, instrument_id.to_string());
    assert!(result.is_ok());

    let result = Portfolio::delete(&mut conn, portfolio_id);
    assert!(result.is_ok());
}
