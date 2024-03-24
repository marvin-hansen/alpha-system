use crate::prelude::{AccountType, ExchangeID};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PortfolioConfig {
    portfolio_id: u32,
    portfolio_description: String,
    portfolio_account_type: AccountType,
    portfolio_account_id: String,
    portfolio_exchange_id: ExchangeID,
    portfolio_currency: String,
    portfolio_cash: f64,
    portfolio_margin: f64,
    portfolio_max_drawdown: f64,
    portfolio_instruments: Vec<String>,
    instrument_max_allocation: f64,
    instrument_max_drawdown: f64,
    //
    portfolio_free_margin: f64,
    portfolio_free_cash: f64,
    portfolio_free_margin_percent: f64,
    portfolio_free_cash_percent: f64,
}

impl PortfolioConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        portfolio_id: u32,
        portfolio_description: String,
        portfolio_account_type: AccountType,
        portfolio_account_id: String,
        portfolio_exchange_id: ExchangeID,
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_margin: f64,
        portfolio_max_drawdown: f64,
        portfolio_instruments: Vec<String>,
        instrument_max_allocation: f64,
        instrument_max_drawdown: f64,
        portfolio_free_margin: f64,
        portfolio_free_cash: f64,
        portfolio_free_margin_percent: f64,
        portfolio_free_cash_percent: f64,
    ) -> Self {
        Self {
            portfolio_id,
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
            portfolio_exchange_id,
            portfolio_currency,
            portfolio_cash,
            portfolio_margin,
            portfolio_max_drawdown,
            portfolio_instruments,
            instrument_max_allocation,
            instrument_max_drawdown,
            portfolio_free_margin,
            portfolio_free_cash,
            portfolio_free_margin_percent,
            portfolio_free_cash_percent,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_cash_portfolio(
        portfolio_id: u32,
        portfolio_description: String,
        portfolio_account_type: AccountType,
        portfolio_account_id: String,
        portfolio_exchange_id: ExchangeID,
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_max_drawdown: f64,
        portfolio_instruments: Vec<String>,
        instrument_max_allocation: f64,
        instrument_max_drawdown: f64,
    ) -> Self {
        Self {
            portfolio_id,
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
            portfolio_exchange_id,
            portfolio_currency,
            portfolio_cash,
            portfolio_margin: 0f64,
            portfolio_max_drawdown,
            portfolio_instruments,
            instrument_max_allocation,
            instrument_max_drawdown,
            portfolio_free_margin: 0f64,
            portfolio_free_cash: portfolio_cash,
            portfolio_free_margin_percent: 0f64,
            portfolio_free_cash_percent: 100.0,
        }
    }
}

impl PortfolioConfig {
    pub fn portfolio_id(&self) -> u32 {
        self.portfolio_id
    }
    pub fn portfolio_description(&self) -> &str {
        &self.portfolio_description
    }
    pub fn portfolio_account_type(&self) -> AccountType {
        self.portfolio_account_type
    }
    pub fn portfolio_account_id(&self) -> &str {
        &self.portfolio_account_id
    }
    pub fn portfolio_exchange_id(&self) -> ExchangeID {
        self.portfolio_exchange_id
    }
    pub fn portfolio_currency(&self) -> &str {
        &self.portfolio_currency
    }
    pub fn portfolio_cash(&self) -> f64 {
        self.portfolio_cash
    }
    pub fn portfolio_margin(&self) -> f64 {
        self.portfolio_margin
    }
    pub fn portfolio_max_drawdown(&self) -> f64 {
        self.portfolio_max_drawdown
    }
    pub fn portfolio_instruments(&self) -> &Vec<String> {
        &self.portfolio_instruments
    }
    pub fn instrument_max_allocation(&self) -> f64 {
        self.instrument_max_allocation
    }
    pub fn instrument_max_drawdown(&self) -> f64 {
        self.instrument_max_drawdown
    }
    pub fn portfolio_free_margin(&self) -> f64 {
        self.portfolio_free_margin
    }
    pub fn portfolio_free_cash(&self) -> f64 {
        self.portfolio_free_cash
    }
    pub fn portfolio_free_margin_percent(&self) -> f64 {
        self.portfolio_free_margin_percent
    }
    pub fn portfolio_free_cash_percent(&self) -> f64 {
        self.portfolio_free_cash_percent
    }
}

impl Display for PortfolioConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "portfolio_id: {}, portfolio_description: {}, portfolio_account_type: {}, \
               portfolio_account_id: {}, portfolio_exchange_id: {}, portfolio_currency: {}, \
               portfolio_cash: {}, portfolio_margin: {:?}, portfolio_max_drawdown: {}, \
               portfolio_instruments: {:?}, instrument_max_allocation: {:?}, \
               instrument_max_drawdown: {:?}, portfolio_free_margin: {:?}, portfolio_free_cash: {:?}, \
               portfolio_free_margin_percent: {:?}, portfolio_free_cash_percent: {:?}",
               self.portfolio_id,
               self.portfolio_description,
               self.portfolio_account_type,
               self.portfolio_account_id,
               self.portfolio_exchange_id,
               self.portfolio_currency,
               self.portfolio_cash,
               self.portfolio_margin,
               self.portfolio_max_drawdown,
               self.portfolio_instruments,
               self.instrument_max_allocation,
               self.instrument_max_drawdown,
               self.portfolio_free_margin,
               self.portfolio_free_cash,
               self.portfolio_free_margin_percent,
               self.portfolio_free_cash_percent
        )
    }
}
