use common_exchange::prelude::PortfolioConfig;
use portfolio_specs::test_portfolio;

pub fn get_all_portfolio_specs() -> Vec<PortfolioConfig> {
    // Update tests if you add more portfolio specs.
    Vec::from([test_portfolio::get_test_portfolio_config()])
}
