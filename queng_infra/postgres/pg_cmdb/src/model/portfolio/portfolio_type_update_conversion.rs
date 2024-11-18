use crate::model::portfolio::UpdatePortfolio;

use crate::model::instrument::Instrument;
use common_exchange::{
    AccountType, Instrument as CommonInstrument, PortfolioConfig as CommonPortfolioConfig,
};
impl UpdatePortfolio {
    pub fn from_common_portfolio(portfolio: &CommonPortfolioConfig) -> UpdatePortfolio {
        UpdatePortfolio {
            portfolio_description: Some(portfolio.portfolio_description().to_string()),
            portfolio_account_type: Some(portfolio.portfolio_account_type().as_i32()),
            portfolio_account_id: Some(portfolio.portfolio_account_id().to_string()),
            portfolio_currency: Some(portfolio.portfolio_currency().to_string()),
            portfolio_cash: Some(portfolio.portfolio_cash()),
            portfolio_margin: Some(portfolio.portfolio_margin()),
            portfolio_max_drawdown: Some(portfolio.portfolio_max_drawdown()),
            instrument_max_allocation: Some(portfolio.instrument_max_allocation()),
            instrument_max_drawdown: Some(portfolio.instrument_max_drawdown()),
            portfolio_free_margin: Some(portfolio.portfolio_free_margin()),
            portfolio_free_cash: Some(portfolio.portfolio_free_cash()),
            portfolio_free_margin_percent: Some(portfolio.portfolio_free_margin_percent()),
            portfolio_free_cash_percent: Some(portfolio.portfolio_free_cash_percent()),
        }
    }

    pub fn to_common_portfolio(
        &self,
        portfolio_id: u32,
        instrument: &[Instrument],
    ) -> CommonPortfolioConfig {
        CommonPortfolioConfig::new(
            portfolio_id,
            self.portfolio_description.clone().unwrap().to_string(),
            AccountType::from(self.portfolio_account_type.unwrap_or_default()),
            self.portfolio_account_id.clone().unwrap().to_string(),
            self.portfolio_currency.clone().unwrap().to_string(),
            self.portfolio_cash.unwrap_or_default(),
            self.portfolio_margin.unwrap_or_default(),
            self.portfolio_max_drawdown.unwrap_or_default(),
            instrument
                .iter()
                .map(|i| i.to_common_instrument())
                .collect(),
            self.instrument_max_allocation.unwrap_or_default(),
            self.instrument_max_drawdown.unwrap_or_default(),
            self.portfolio_free_margin.unwrap_or_default(),
            self.portfolio_free_cash.unwrap_or_default(),
            self.portfolio_free_margin_percent.unwrap_or_default(),
            self.portfolio_free_cash_percent.unwrap_or_default(),
        )
    }

    pub fn to_common_portfolio_with_instruments(
        &self,
        portfolio_id: u32,
        instruments: Vec<CommonInstrument>,
    ) -> CommonPortfolioConfig {
        CommonPortfolioConfig::new(
            portfolio_id,
            self.portfolio_description.clone().unwrap().to_string(),
            AccountType::from(self.portfolio_account_type.unwrap_or_default()),
            self.portfolio_account_id.clone().unwrap().to_string(),
            self.portfolio_currency.clone().unwrap().to_string(),
            self.portfolio_cash.unwrap_or_default(),
            self.portfolio_margin.unwrap_or_default(),
            self.portfolio_max_drawdown.unwrap_or_default(),
            instruments,
            self.instrument_max_allocation.unwrap_or_default(),
            self.instrument_max_drawdown.unwrap_or_default(),
            self.portfolio_free_margin.unwrap_or_default(),
            self.portfolio_free_cash.unwrap_or_default(),
            self.portfolio_free_margin_percent.unwrap_or_default(),
            self.portfolio_free_cash_percent.unwrap_or_default(),
        )
    }
}
