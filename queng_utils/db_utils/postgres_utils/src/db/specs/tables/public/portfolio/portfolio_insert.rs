use common_exchange::prelude::PortfolioConfig;
use common_pg_queries::portfolio_insert;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Inserts a new portfolio into the `public.portfolio` table.
    ///
    /// This method takes a `portfolio` and inserts a new portfolio into the `public.portfolio` table.
    /// The method generates an SQL query using the `build_insert_portfolio_query` method and executes it using the `execute_query` method.
    /// If the query is successful, the method returns `Ok(portfolio_id)`, where `portfolio_id` is the ID of the inserted portfolio.
    /// Otherwise, it returns an `Err` containing a `PostgresUtilError` with a message indicating the failure.
    ///
    /// # Arguments
    ///
    /// * `portfolio` - The portfolio to be inserted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(portfolio_id)` if the portfolio is successfully inserted, where `portfolio_id` is the ID of the inserted portfolio.
    /// Otherwise, it returns an `Err` containing a `PostgresUtilError` with a message indicating the failure.
    ///
    /// # Errors
    ///
    /// Returns a `PostgresUtilError` if the insertion fails.
    ///
    pub async fn insert_portfolio(&self, data: &PortfolioConfig) -> Result<(), PostgresUtilError> {
        self.dbg_print("insert_portfolio");

        let query = portfolio_insert::build_insert_portfolio_query(data);
        let portfolio_id = match self.execute_insert_query(&query).await {
            Ok(id) => id,
            Err(err) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to insert portfolio: {}",
                    err
                )))
            }
        };

        for instrument in data.portfolio_instruments() {
            let instrument_id = match self.insert_instrument(instrument).await {
                Ok(id) => id,
                Err(err) => {
                    return Err(PostgresUtilError::new(format!(
                        "Failed to insert instrument: {}",
                        err
                    )))
                }
            };

            match self
                .insert_portfolio_instrument(portfolio_id, instrument_id)
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    return Err(PostgresUtilError::new(format!(
                        "Failed to insert portfolio_instrument: {}",
                        err
                    )))
                }
            };
        }

        Ok(())
    }
}
