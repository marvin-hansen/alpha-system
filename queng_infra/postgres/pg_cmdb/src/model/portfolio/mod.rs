use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

mod portfolio_impl;
mod portfolio_instrument_joins_impl;
mod portfolio_type_conversion;
mod portfolio_type_create_conversion;
mod portfolio_type_update_conversion;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::cmdb::portfolio, primary_key(portfolio_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Portfolio {
    pub portfolio_id: i32,
    pub portfolio_description: String,
    pub portfolio_account_type: i32,
    pub portfolio_account_id: String,
    pub portfolio_currency: String,
    pub portfolio_cash: f64,
    pub portfolio_margin: f64,
    pub portfolio_max_drawdown: f64,
    pub instrument_max_allocation: f64,
    pub instrument_max_drawdown: f64,
    pub portfolio_free_margin: f64,
    pub portfolio_free_cash: f64,
    pub portfolio_free_margin_percent: f64,
    pub portfolio_free_cash_percent: f64,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::cmdb::portfolio)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreatePortfolio {
    pub portfolio_id: i32,
    pub portfolio_description: String,
    pub portfolio_account_type: i32,
    pub portfolio_account_id: String,
    pub portfolio_currency: String,
    pub portfolio_cash: f64,
    pub portfolio_margin: f64,
    pub portfolio_max_drawdown: f64,
    pub instrument_max_allocation: f64,
    pub instrument_max_drawdown: f64,
    pub portfolio_free_margin: f64,
    pub portfolio_free_cash: f64,
    pub portfolio_free_margin_percent: f64,
    pub portfolio_free_cash_percent: f64,
}

impl CreatePortfolio {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        portfolio_id: i32,
        portfolio_description: String,
        portfolio_account_type: i32,
        portfolio_account_id: String,
        portfolio_currency: String,
        portfolio_cash: f64,
        portfolio_margin: f64,
        portfolio_max_drawdown: f64,
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
            instrument_max_allocation,
            instrument_max_drawdown,
            portfolio_free_margin,
            portfolio_free_cash,
            portfolio_free_margin_percent,
            portfolio_free_cash_percent,
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::cmdb::portfolio)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatePortfolio {
    pub portfolio_description: Option<String>,
    pub portfolio_account_type: Option<i32>,
    pub portfolio_account_id: Option<String>,
    pub portfolio_currency: Option<String>,
    pub portfolio_cash: Option<f64>,
    pub portfolio_margin: Option<f64>,
    pub portfolio_max_drawdown: Option<f64>,
    pub instrument_max_allocation: Option<f64>,
    pub instrument_max_drawdown: Option<f64>,
    pub portfolio_free_margin: Option<f64>,
    pub portfolio_free_cash: Option<f64>,
    pub portfolio_free_margin_percent: Option<f64>,
    pub portfolio_free_cash_percent: Option<f64>,
}

impl UpdatePortfolio {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        portfolio_description: Option<String>,
        portfolio_account_type: Option<i32>,
        portfolio_account_id: Option<String>,
        portfolio_currency: Option<String>,
        portfolio_cash: Option<f64>,
        portfolio_margin: Option<f64>,
        portfolio_max_drawdown: Option<f64>,
        instrument_max_allocation: Option<f64>,
        instrument_max_drawdown: Option<f64>,
        portfolio_free_margin: Option<f64>,
        portfolio_free_cash: Option<f64>,
        portfolio_free_margin_percent: Option<f64>,
        portfolio_free_cash_percent: Option<f64>,
    ) -> Self {
        Self {
            portfolio_description,
            portfolio_account_type,
            portfolio_account_id,
            portfolio_currency,
            portfolio_cash,
            portfolio_margin,
            portfolio_max_drawdown,
            instrument_max_allocation,
            instrument_max_drawdown,
            portfolio_free_margin,
            portfolio_free_cash,
            portfolio_free_margin_percent,
            portfolio_free_cash_percent,
        }
    }
}
