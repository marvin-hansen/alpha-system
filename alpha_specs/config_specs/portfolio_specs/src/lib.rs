/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod test_portfolio;

use common_exchange::PortfolioConfig;

#[must_use]
pub fn get_test_update_portfolio_config() -> PortfolioConfig {
    test_portfolio::get_test_update_portfolio_config()
}

#[must_use]
pub fn get_test_update_error_portfolio_config() -> PortfolioConfig {
    test_portfolio::get_test_update_error_portfolio_config()
}

#[must_use]
pub fn get_test_portfolio_config() -> PortfolioConfig {
    test_portfolio::get_test_portfolio_config()
}

#[must_use]
pub fn get_all_portfolio_specs() -> Vec<PortfolioConfig> {
    // Update tests if you add more portfolio specs.
    Vec::from([test_portfolio::get_test_portfolio_config()])
}
