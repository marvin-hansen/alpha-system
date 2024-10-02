use proto_cmdb::proto::ProtoPortfolioConfig;

#[test]
fn test_proto_portfolio_config() {
    let proto = ProtoPortfolioConfig {
        portfolio_id: -1, // Invalid ID
        portfolio_description: "Test Portfolio".to_string(),
        portfolio_account_type: 1,
        portfolio_account_id: "12345".to_string(),
        portfolio_currency: "USD".to_string(),
        portfolio_cash: 1000.0,
        portfolio_margin: 500.0,
        portfolio_max_drawdown: 10.0,
        portfolio_instruments: vec![],
        instrument_max_allocation: 50.0,
        instrument_max_drawdown: 5.0,
        portfolio_free_margin: 200.0,
        portfolio_free_cash: 300.0,
        portfolio_free_margin_percent: 20.0,
        portfolio_free_cash_percent: 30.0,
    };

    assert_eq!(proto.portfolio_id, -1);
    assert_eq!(proto.portfolio_description, "Test Portfolio");
    assert_eq!(proto.portfolio_account_type, 1);
    assert_eq!(proto.portfolio_account_id, "12345");
    assert_eq!(proto.portfolio_currency, "USD");
    assert_eq!(proto.portfolio_cash, 1000.0);
    assert_eq!(proto.portfolio_margin, 500.0);
    assert_eq!(proto.portfolio_max_drawdown, 10.0);
    assert_eq!(proto.instrument_max_allocation, 50.0);
    assert_eq!(proto.instrument_max_drawdown, 5.0);
    assert_eq!(proto.portfolio_free_margin, 200.0);
    assert_eq!(proto.portfolio_free_cash, 300.0);
    assert_eq!(proto.portfolio_free_margin_percent, 20.0);
    assert_eq!(proto.portfolio_free_cash_percent, 30.0);
}
