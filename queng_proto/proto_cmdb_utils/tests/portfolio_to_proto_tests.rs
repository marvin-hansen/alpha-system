use common_exchange::{AccountType, Instrument, PortfolioConfig};
use proto_cmdb_utils::portfolio_proto_utils::{instrument_to_proto, portfolio_config_to_proto};

#[test]
fn test_portfolio_config_to_proto_conversion() {
    let portfolio_config = PortfolioConfig::new(
        1,
        "Test Portfolio".to_string(),
        AccountType::Spot,
        "ACC123".to_string(),
        "USD".to_string(),
        1000.0,
        500.0,
        200.0,
        vec![Instrument::new(
            "AAPL".to_string(),
            "Equity".to_string(),
            "NASDAQ".to_string(),
            "AAPL".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some("BBG000B9XRY4".to_string()),
        )],
        50.0,
        10.0,
        300.0,
        700.0,
        30.0,
        70.0,
    );

    let proto_config = portfolio_config_to_proto(portfolio_config).unwrap();

    assert_eq!(proto_config.portfolio_id, 1);
    assert_eq!(proto_config.portfolio_description, "Test Portfolio");
    assert_eq!(
        proto_config.portfolio_account_type,
        AccountType::Spot.as_i32()
    );
    assert_eq!(proto_config.portfolio_account_id, "ACC123");
    assert_eq!(proto_config.portfolio_currency, "USD");
    assert_eq!(proto_config.portfolio_cash, 1000.0);
    assert_eq!(proto_config.portfolio_margin, 500.0);
    assert_eq!(proto_config.portfolio_max_drawdown, 200.0);
    assert_eq!(proto_config.instrument_max_allocation, 50.0);
    assert_eq!(proto_config.instrument_max_drawdown, 10.0);
    assert_eq!(proto_config.portfolio_free_margin, 300.0);
    assert_eq!(proto_config.portfolio_free_cash, 700.0);
    assert_eq!(proto_config.portfolio_free_margin_percent, 30.0);
    assert_eq!(proto_config.portfolio_free_cash_percent, 70.0);
    assert_eq!(proto_config.portfolio_instruments.len(), 1);
    assert_eq!(
        proto_config.portfolio_instruments[0].instrument_code,
        "AAPL"
    );
}

#[test]
fn test_portfolio_config_to_proto_empty_instruments() {
    let portfolio_config = PortfolioConfig::new(
        1,
        "Test Portfolio".to_string(),
        AccountType::Spot,
        "ACC123".to_string(),
        "USD".to_string(),
        1000.0,
        500.0,
        200.0,
        vec![],
        50.0,
        10.0,
        300.0,
        700.0,
        30.0,
        70.0,
    );

    let proto_config = portfolio_config_to_proto(portfolio_config).unwrap();

    assert_eq!(proto_config.portfolio_instruments.len(), 0);
}

#[test]
fn test_instrument_to_proto() {
    let instruments = vec![
        Instrument::new(
            "AAPL".to_string(),
            "Equity".to_string(),
            "NASDAQ".to_string(),
            "AAPL".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some("BBG000B9XRY4".to_string()),
        ),
        Instrument::new(
            "GOOGL".to_string(),
            "Equity".to_string(),
            "NASDAQ".to_string(),
            "GOOGL".to_string(),
            "USD".to_string(),
            "USD".to_string(),
            Some("BBG000B9XRY4".to_string()),
        ),
    ];

    let proto_instruments = instrument_to_proto(instruments.clone());

    assert_eq!(proto_instruments.len(), 2);

    for (i, proto) in proto_instruments.iter().enumerate() {
        assert_eq!(proto.instrument_code, instruments[i].code());
        assert_eq!(proto.instrument_class, instruments[i].class());
        assert_eq!(proto.exchange_code, instruments[i].exchange_code());
        assert_eq!(
            proto.exchange_pair_code,
            instruments[i].exchange_pair_code()
        );
        assert_eq!(proto.base_asset, instruments[i].base_asset());
        assert_eq!(proto.quote_asset, instruments[i].quote_asset());
    }
}
