use common_exchange::prelude::AccountType;
use portfolio_specs::prelude::test_portfolio::get_test_portfolio_config;

#[test]
fn portfolio_config_portfolio_id_returns_expected_value() {
    let portfolio_id = 1;
    let portfolio_config = get_test_portfolio_config();
    assert_eq!(portfolio_config.portfolio_id(), portfolio_id);
}

#[test]
fn test_portfolio_id() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_id(), 1);
}

#[test]
fn test_portfolio_description() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_description(), "Test portfolio");
}

#[test]
fn test_portfolio_account_type() {
    let account_type = AccountType::Spot;
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_account_type(), account_type);
}

#[test]
fn test_portfolio_account_id() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_account_id(), "cash_account");
}

#[test]
fn test_portfolio_currency() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_currency(), "USD");
}

#[test]
fn test_portfolio_cash() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_cash(), 1000.0);
}

#[test]
fn test_portfolio_margin() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_margin(), 0.0f64);
}

#[test]
fn test_portfolio_max_drawdown() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_max_drawdown(), 15.0);
}

#[test]
fn test_instrument_max_allocation() {
    let instrument_max_allocation = 0.05f64;
    let config = get_test_portfolio_config();
    assert_eq!(
        config.instrument_max_allocation(),
        instrument_max_allocation
    );
}

#[test]
fn test_instrument_max_drawdown() {
    let max_drawdown = 10.0;
    let config = get_test_portfolio_config();
    assert_eq!(config.instrument_max_drawdown(), max_drawdown);
}

#[test]
fn test_portfolio_free_margin() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_free_margin(), 0.0f64);
}

#[test]
fn test_portfolio_free_cash() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_free_cash(), (1000.0));
}

#[test]
fn test_portfolio_free_margin_percent() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_free_margin_percent(), 0f64);
}

#[test]
fn test_portfolio_free_cash_percent() {
    let config = get_test_portfolio_config();
    assert_eq!(config.portfolio_free_cash_percent(), 100.0f64);
}
