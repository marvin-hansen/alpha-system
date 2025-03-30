/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_exchange::{AccountType, Instrument, PortfolioConfig};

pub fn get_test_portfolio_config() -> PortfolioConfig {
    let portfolio_id = 1;
    let portfolio_description = "Test portfolio".to_string();
    let portfolio_account_type = AccountType::Spot;
    let portfolio_account_id = "cash_account".to_string();
    let portfolio_currency = "USD".to_string();
    let portfolio_cash_balance = 1000.0;
    let portfolio_max_drawdown = 15.0;
    let portfolio_instruments = vec![get_test_instrument()];
    let instrument_max_allocation = 0.05;
    let instrument_max_drawdown = 10.0;

    PortfolioConfig::new_cash_portfolio(
        portfolio_id,
        portfolio_description,
        portfolio_account_type,
        portfolio_account_id,
        portfolio_currency,
        portfolio_cash_balance,
        portfolio_max_drawdown,
        portfolio_instruments,
        instrument_max_allocation,
        instrument_max_drawdown,
    )
}

pub fn get_test_update_portfolio_config() -> PortfolioConfig {
    let portfolio_id = 1;
    let portfolio_description = "Test portfolio".to_string();
    let portfolio_account_type = AccountType::Spot;
    let portfolio_account_id = "cash_account".to_string();
    let portfolio_currency = "USD".to_string();
    let portfolio_cash_balance = 10000.0;
    let portfolio_max_drawdown = 10.0;
    let portfolio_instruments = vec![get_test_instrument()];
    let instrument_max_allocation = 0.05;
    let instrument_max_drawdown = 10.0;

    PortfolioConfig::new_cash_portfolio(
        portfolio_id,
        portfolio_description,
        portfolio_account_type,
        portfolio_account_id,
        portfolio_currency,
        portfolio_cash_balance,
        portfolio_max_drawdown,
        portfolio_instruments,
        instrument_max_allocation,
        instrument_max_drawdown,
    )
}

pub fn get_test_update_error_portfolio_config() -> PortfolioConfig {
    PortfolioConfig::new_cash_portfolio(
        876,
        String::new(),
        AccountType::Spot,
        String::new(),
        String::new(),
        0.0,
        0.0,
        vec![get_test_instrument()],
        0.0,
        0.0,
    )
}

fn get_test_instrument() -> Instrument {
    Instrument::new(
        "ens-krw".to_string(),
        "spot".to_string(),
        "cbse".to_string(),
        "KRW-ENS".to_string(),
        "ens".to_string(),
        "krw".to_string(),
        None,
    )
}
