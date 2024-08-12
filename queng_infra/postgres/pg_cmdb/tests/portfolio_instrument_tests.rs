use crate::util;
use pg_cmdb::model::instrument::{CreateInstrument, Instrument};
use pg_cmdb::model::portfolio::{CreatePortfolio, Portfolio};
use pg_cmdb::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};

#[test]
fn test_portfolio_instrument() {
    let pool = util::postgres_connection_pool();

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

    let mut conn = &mut pool.get().unwrap();

    let result = Portfolio::create(&mut conn, &create_portfolio);
    assert!(result.is_ok());

    let instrument_id = "test42";

    let create_instrument = CreateInstrument {
        code: instrument_id.to_string(),
        class: "test".to_string(),
        exchange_code: "test".to_string(),
        exchange_pair_code: "test".to_string(),
        base_asset: "test".to_string(),
        quote_asset: "test".to_string(),
        instrument_figi: Some("test".to_string()),
    };

    let result = Instrument::create(&mut conn, &create_instrument);
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
