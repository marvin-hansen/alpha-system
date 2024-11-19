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
    /// * `QueryResult<Self>` - The newly created portfolio instrument if successful.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The insert operation fails (e.g., due to constraint violations)
    /// * The portfolio or instrument referenced in `item` doesn't exist
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
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The query execution fails
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
    ///   Returns an empty vector if no instruments are found for the portfolio.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The query execution fails
    /// * The portfolio ID does not exist
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
    /// * `QueryResult<usize>` - The number of rows affected by the delete operation.
    ///   Returns `Ok(0)` if no matching record was found to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The database connection fails
    /// * The delete operation fails
    /// * The query execution fails
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
