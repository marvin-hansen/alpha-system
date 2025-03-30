/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

mod portfolio_instrument_impl;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(crate::model::portfolio::Portfolio))]
#[diesel(belongs_to(crate::model::instrument::Instrument))]
#[diesel(table_name=crate::schema::cmdb::portfolio_instrument)]
#[diesel(primary_key(portfolio_id, instrument_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PortfolioInstrument {
    pub portfolio_id: i32,
    pub instrument_id: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=crate::schema::cmdb::portfolio_instrument)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreatePortfolioInstrument {
    pub portfolio_id: i32,
    pub instrument_id: String,
}

impl CreatePortfolioInstrument {
    #[must_use]
    pub const fn new(portfolio_id: i32, instrument_id: String) -> Self {
        Self {
            portfolio_id,
            instrument_id,
        }
    }
}
