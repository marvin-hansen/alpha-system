use crate::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use crate::schema::cmdb::portfolio_instrument::dsl::*;
use crate::Connection;
use diesel::{
    insert_into, result::Error, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl,
    SelectableHelper,
};

impl PortfolioInstrument {
    pub fn create(db: &mut Connection, item: &CreatePortfolioInstrument) -> QueryResult<Self> {
        insert_into(portfolio_instrument)
            .values(item)
            .returning(PortfolioInstrument::as_returning())
            .get_result::<Self>(db)
    }

    pub fn check_if_exists(
        db: &mut Connection,
        param_portfolio_id: i32,
        param_instrument_id: String,
    ) -> QueryResult<bool> {
        match portfolio_instrument
            .find((param_portfolio_id, param_instrument_id))
            .first::<Self>(db)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn read_instruments_for_portfolio(
        db: &mut Connection,
        param_portfolio_id: i32,
    ) -> QueryResult<Vec<PortfolioInstrument>> {
        match portfolio_instrument
            .filter(portfolio_id.eq(param_portfolio_id))
            .load(db)
        {
            Ok(v) => {
                if v.is_empty() {
                    Err(Error::NotFound)
                } else {
                    Ok(v)
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(
        db: &mut Connection,
        param_portfolio_id: i32,
        param_instrument_id: String,
    ) -> QueryResult<usize> {
        diesel::delete(
            portfolio_instrument
                .filter(portfolio_id.eq(param_portfolio_id))
                .filter(instrument_id.eq(param_instrument_id)),
        )
        .execute(db)
    }
}
