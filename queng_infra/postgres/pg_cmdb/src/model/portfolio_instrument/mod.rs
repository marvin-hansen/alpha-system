use crate::schema::cmdb::{instrument, portfolio, portfolio_instrument};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

mod portfolio_instrument_impl;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(portfolio::dsl::portfolio))]
#[diesel(belongs_to(instrument::dsl::instrument))]
#[diesel(table_name=portfolio_instrument)]
#[diesel(primary_key(portfolio_id, instrument_id))]
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
