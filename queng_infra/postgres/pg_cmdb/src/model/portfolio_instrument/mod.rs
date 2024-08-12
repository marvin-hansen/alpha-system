use crate::schema::cmdb::portfolio_instrument;
use diesel::{Insertable, Queryable, Selectable};

mod portfolio_instrument_impl;

#[derive(Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(belongs_to(portfolio))]
#[diesel(belongs_to(instrument))]
#[diesel(table_name=portfolio_instrument, primary_key(portfolio_id,instrument_id))]
pub struct PortfolioInstrument {
    pub portfolio_id: i32,
    pub instrument_id: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=portfolio_instrument)]
pub struct CreatePortfolioInstrument {
    pub portfolio_id: i32,
    pub instrument_id: String,
}

impl CreatePortfolioInstrument {
    pub fn new(portfolio_id: i32, instrument_id: String) -> Self {
        Self {
            portfolio_id,
            instrument_id,
        }
    }
}
