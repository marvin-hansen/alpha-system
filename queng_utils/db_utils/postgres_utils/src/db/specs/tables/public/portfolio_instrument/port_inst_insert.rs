use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Inserts a portfolio_instrument into the public.portfolio_instrument table.
    ///
    /// This method takes a `PortfolioInstrument` object and inserts a new portfolio_instrument into the
    /// public.portfolio_instrument table. The method generates an SQL query using the `build_insert_query`
    /// method and executes it using the `execute_query` method. If the query is successful,
    /// the method returns `Ok(())`. Otherwise, it returns an `Err` containing a
    /// `PostgresUtilError` with a message indicating the failure.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a `PortfolioInstrument` object containing the data for the
    ///   new portfolio_instrument.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the portfolio_instrument is successfully inserted,
    /// or an `Err` containing a `PostgresUtilError` with a message indicating the failure.
    ///
    pub async fn insert_portfolio_instrument(
        &self,
        portfolio_id: u64,
        instrument_id: u64,
    ) -> Result<(), PostgresUtilError> {
        self.dbg_print("insert_portfolio_instrument");

        let query = self.build_insert_portfolio_instrument_query(portfolio_id, instrument_id);
        // println!("query: {}", query);
        return match self.execute_query(&query).await {
            Ok(_) => Ok(()),
            Err(err) => Err(PostgresUtilError::new(format!(
                "Failed to insert portfolio_instrument due error: {}",
                err
            ))),
        };
    }

    fn build_insert_portfolio_instrument_query(
        &self,
        portfolio_id: u64,
        instrument_id: u64,
    ) -> String {
        format!(
            "INSERT INTO public.portfolio_instrument (portfolio_id, instrument_id) VALUES ({}, {})",
            portfolio_id, instrument_id
        )
    }
}
