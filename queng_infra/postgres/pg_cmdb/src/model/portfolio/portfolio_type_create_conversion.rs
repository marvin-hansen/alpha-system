use crate::model::portfolio::CreatePortfolio;

use common_exchange::PortfolioConfig as CommonPortfolioConfig;

impl CreatePortfolio {
    pub fn from_common_portfolio(portfolio: &CommonPortfolioConfig) -> CreatePortfolio {
        CreatePortfolio {
            portfolio_id: portfolio.portfolio_id() as i32,
            portfolio_description: portfolio.portfolio_description().to_string(),
            portfolio_account_type: portfolio.portfolio_account_type().as_i32(),
            portfolio_account_id: portfolio.portfolio_account_id().to_string(),
            portfolio_currency: portfolio.portfolio_currency().to_string(),
            portfolio_cash: portfolio.portfolio_cash(),
            portfolio_margin: portfolio.portfolio_margin(),
            portfolio_max_drawdown: portfolio.portfolio_max_drawdown(),
            instrument_max_allocation: portfolio.instrument_max_allocation(),
            instrument_max_drawdown: portfolio.instrument_max_drawdown(),
            portfolio_free_margin: portfolio.portfolio_free_margin(),
            portfolio_free_cash: portfolio.portfolio_free_cash(),
            portfolio_free_margin_percent: portfolio.portfolio_free_margin_percent(),
            portfolio_free_cash_percent: portfolio.portfolio_free_cash_percent(),
        }
    }
}
