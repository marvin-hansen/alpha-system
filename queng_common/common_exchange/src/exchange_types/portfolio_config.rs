use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::prelude::{AccountType, Instrument};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PortfolioConfig {
    portfolio_id: u32,
    portfolio_description: String,
    portfolio_account_type: AccountType,
    portfolio_account_id: String,
    portfolio_currency: String,
    portfolio_cash: f64,
    portfolio_margin: f64,
    portfolio_max_drawdown: f64,
    portfolio_instruments: Vec<Instrument>,
    instrument_max_allocation: f64,
    instrument_max_drawdown: f64,
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
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_margin: f64,
        portfolio_max_drawdown: f64,
        portfolio_instruments: Vec<Instrument>,
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
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_max_drawdown: f64,
        portfolio_instruments: Vec<Instrument>,
        instrument_max_allocation: f64,
        instrument_max_drawdown: f64,
    ) -> Self {
        Self {
            portfolio_id,
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
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
    pub fn portfolio_instruments(&self) -> &Vec<Instrument> {
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

impl PortfolioConfig {
    pub fn from_sql_row(row: &Row, portfolio_instruments: Vec<Instrument>) -> Self {
        let portfolio_id = row.get::<usize, i32>(0);
        let portfolio_description = row.get::<usize, String>(1);
        let portfolio_at = row.get::<usize, i16>(2);
        let portfolio_account_type = AccountType::from(portfolio_at);
        let portfolio_account_id = row.get::<usize, String>(3);
        let portfolio_currency = row.get::<usize, String>(4);
        let portfolio_cash = row.get::<usize, f64>(5);
        let portfolio_margin = row.get::<usize, f64>(6);
        let portfolio_max_drawdown = row.get::<usize, f64>(7);
        let instrument_max_allocation = row.get::<usize, f64>(8);
        let instrument_max_drawdown = row.get::<usize, f64>(9);
        let portfolio_free_margin = row.get::<usize, f64>(10);
        let portfolio_free_cash = row.get::<usize, f64>(11);
        let portfolio_free_margin_percent = row.get::<usize, f64>(12);
        let portfolio_free_cash_percent = row.get::<usize, f64>(13);

        PortfolioConfig::new(
            portfolio_id as u32,
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
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
        )
    }
}

impl Display for PortfolioConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "portfolio_id: {}, portfolio_description: {}, portfolio_account_type: {}, \
               portfolio_account_id: {}, portfolio_currency: {}, \
               portfolio_cash: {}, portfolio_margin: {:?}, portfolio_max_drawdown: {}, \
               portfolio_instruments: {:?}, instrument_max_allocation: {:?}, \
               instrument_max_drawdown: {:?}, portfolio_free_margin: {:?}, portfolio_free_cash: {:?}, \
               portfolio_free_margin_percent: {:?}, portfolio_free_cash_percent: {:?}",
               self.portfolio_id,
               self.portfolio_description,
               self.portfolio_account_type,
               self.portfolio_account_id,
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
