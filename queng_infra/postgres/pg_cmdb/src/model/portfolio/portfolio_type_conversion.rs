/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::portfolio::Portfolio;

use crate::model::instrument::Instrument;
use common_exchange::{
    AccountType, Instrument as CommonInstrument, PortfolioConfig as CommonPortfolioConfig,
};

impl Portfolio {
    #[must_use]
    pub fn from_common_portfolio(portfolio: CommonPortfolioConfig) -> Self {
        Self {
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

    #[must_use]
    pub fn to_common_portfolio(&self, instruments: &[Instrument]) -> CommonPortfolioConfig {
        CommonPortfolioConfig::new(
            self.portfolio_id as u32,
            self.portfolio_description.to_string(),
            AccountType::from(self.portfolio_account_type),
            self.portfolio_account_id.to_string(),
            self.portfolio_currency.to_string(),
            self.portfolio_cash,
            self.portfolio_margin,
            self.portfolio_max_drawdown,
            instruments
                .iter()
                .map(super::super::instrument::Instrument::to_common_instrument)
                .collect(),
            self.instrument_max_allocation,
            self.instrument_max_drawdown,
            self.portfolio_free_margin,
            self.portfolio_free_cash,
            self.portfolio_free_margin_percent,
            self.portfolio_free_cash_percent,
        )
    }

    #[must_use]
    pub fn to_common_portfolio_with_common_instruments(
        &self,
        instruments: Vec<CommonInstrument>,
    ) -> CommonPortfolioConfig {
        CommonPortfolioConfig::new(
            self.portfolio_id as u32,
            self.portfolio_description.to_string(),
            AccountType::from(self.portfolio_account_type),
            self.portfolio_account_id.to_string(),
            self.portfolio_currency.to_string(),
            self.portfolio_cash,
            self.portfolio_margin,
            self.portfolio_max_drawdown,
            instruments,
            self.instrument_max_allocation,
            self.instrument_max_drawdown,
            self.portfolio_free_margin,
            self.portfolio_free_cash,
            self.portfolio_free_margin_percent,
            self.portfolio_free_cash_percent,
        )
    }
}
