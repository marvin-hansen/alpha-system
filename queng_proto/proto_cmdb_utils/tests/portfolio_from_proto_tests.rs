use common_exchange::AccountType;
use proto_cmdb::proto::{ProtoInstrument, ProtoPortfolioConfig};
use proto_cmdb_utils::portfolio_proto_utils::{instrument_from_proto, portfolio_config_from_proto};

#[test]
fn test_portfolio_config_from_proto_valid_conversion() {
    let proto = ProtoPortfolioConfig {
        portfolio_id: 1,
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

    let result = portfolio_config_from_proto(proto);
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.portfolio_id(), 1);
    assert_eq!(config.portfolio_description(), "Test Portfolio");
    assert_eq!(config.portfolio_account_type(), AccountType::Spot);
    assert_eq!(config.portfolio_account_id(), "12345");
    assert_eq!(config.portfolio_currency(), "USD");
    assert_eq!(config.portfolio_cash(), 1000.0);
    assert_eq!(config.portfolio_margin(), 500.0);
    assert_eq!(config.portfolio_max_drawdown(), 10.0);
    assert_eq!(config.instrument_max_allocation(), 50.0);
    assert_eq!(config.instrument_max_drawdown(), 5.0);
    assert_eq!(config.portfolio_free_margin(), 200.0);
    assert_eq!(config.portfolio_free_cash(), 300.0);
    assert_eq!(config.portfolio_free_margin_percent(), 20.0);
    assert_eq!(config.portfolio_free_cash_percent(), 30.0);
}

#[test]
fn test_portfolio_config_from_proto_invalid_portfolio_id() {
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

    let result = portfolio_config_from_proto(proto);
    assert!(result.is_err());
}

#[test]
fn test_portfolio_config_from_proto_invalid_account_type() {
    let proto = ProtoPortfolioConfig {
        portfolio_id: 1,
        portfolio_description: "Test Portfolio".to_string(),
        portfolio_account_type: -21, // Invalid account type
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

    let result = portfolio_config_from_proto(proto);
    assert!(result.is_err());
}

#[test]
fn test_instrument_from_proto_conversion() {
    let proto_instruments = vec![
        ProtoInstrument {
            instrument_code: String::new(),
            exchange_code: "NYSE".to_string(),
            instrument_class: "Equity".to_string(),
            exchange_pair_code: "AAPL".to_string(),
            base_asset: "USD".to_string(),
            quote_asset: "USD".to_string(),
            instrument_figi: None,
        },
        ProtoInstrument {
            instrument_code: String::new(),
            exchange_code: "NASDAQ".to_string(),
            instrument_class: "Equity".to_string(),
            exchange_pair_code: "GOOGL".to_string(),
            base_asset: "USD".to_string(),
            quote_asset: "USD".to_string(),
            instrument_figi: None,
        },
    ];

    let instruments = instrument_from_proto(proto_instruments.clone());

    assert_eq!(instruments.len(), proto_instruments.len());
    for (i, proto) in proto_instruments.iter().enumerate() {
        assert_eq!(instruments[i].code(), proto.instrument_code);
        assert_eq!(instruments[i].exchange_code(), proto.exchange_code);
        assert_eq!(instruments[i].class(), proto.instrument_class);
        assert_eq!(
            instruments[i].exchange_pair_code(),
            proto.exchange_pair_code
        );
        assert_eq!(instruments[i].base_asset(), proto.base_asset);
        assert_eq!(instruments[i].quote_asset(), proto.quote_asset);
    }
}

#[test]
fn test_instrument_from_proto_empty_vector() {
    let proto_instruments: Vec<ProtoInstrument> = Vec::new();
    let instruments = instrument_from_proto(proto_instruments);
    assert!(instruments.is_empty());
}

#[test]
fn test_instrument_from_proto_with_empty_fields() {
    let proto_instruments = vec![ProtoInstrument {
        instrument_code: String::new(),
        exchange_code: String::new(),
        instrument_class: String::new(),
        exchange_pair_code: String::new(),
        base_asset: String::new(),
        quote_asset: String::new(),
        instrument_figi: None,
    }];

    let instruments = instrument_from_proto(proto_instruments.clone());

    assert_eq!(instruments.len(), proto_instruments.len());
    for (i, proto) in proto_instruments.iter().enumerate() {
        assert_eq!(instruments[i].exchange_code(), proto.exchange_code);
        assert_eq!(instruments[i].class(), proto.instrument_class);
        assert_eq!(
            instruments[i].exchange_pair_code(),
            proto.exchange_pair_code
        );
        assert_eq!(instruments[i].base_asset(), proto.base_asset);
        assert_eq!(instruments[i].quote_asset(), proto.quote_asset);
    }
}
