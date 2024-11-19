use crate::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use crate::schema::cmdb::portfolio_instrument::dsl::{
    instrument_id, portfolio_id, portfolio_instrument,
};
use crate::Connection;
use diesel::{
    insert_into, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};

impl PortfolioInstrument {
    ///
    /// Inserts a new portfolio instrument into the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `item` - A reference to the `CreatePortfolioInstrument` item to be inserted.
    ///
    /// # Returns
    ///
    /// * `QueryResult<Self>` - The result of the insertion operation.
    ///
    pub fn create(db: &mut Connection, item: &CreatePortfolioInstrument) -> QueryResult<Self> {
        insert_into(portfolio_instrument)
            .values(item)
            .returning(Self::as_returning())
            .get_result::<Self>(db)
    }

    ///
    /// Checks if a portfolio instrument exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `param_portfolio_id` - The portfolio ID to check.
    /// * `param_instrument_id` - The instrument ID to check.
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - `true` if the portfolio instrument exists, `false` otherwise.
    ///
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

    ///
    /// Reads the instruments associated with a specific portfolio from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `param_portfolio_id` - The ID of the portfolio to retrieve instruments for.
    ///
    /// # Returns
    ///
    /// * `QueryResult<Vec<PortfolioInstrument>>` - A vector containing the portfolio instruments if successful.
    /// * Returns an error if the operation fails.
    ///
    /// Note, when the portfolio has no instruments, an empty vector is returned.
    ///
    pub fn read_instruments_for_portfolio(
        db: &mut Connection,
        param_portfolio_id: i32,
    ) -> QueryResult<Vec<Self>> {
        match portfolio_instrument
            .filter(portfolio_id.eq(param_portfolio_id))
            .load(db)
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    ///
    /// Deletes a portfolio instrument from the database
    /// based on the provided portfolio ID and instrument ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the database connection.
    /// * `param_portfolio_id` - The ID of the portfolio associated with the instrument.
    /// * `param_instrument_id` - The ID of the instrument to be deleted.
    ///
    /// # Returns
    ///
    /// A `QueryResult<usize>` indicating the number of rows affected by the delete operation.
    ///
    /// Note, delete only returns an error when either the database connection or the query fails.
    /// If no rows are affected, the result will be `Ok(0)`.
    ///
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
