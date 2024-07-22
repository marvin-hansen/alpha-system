use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops all the relation tables related to specifications.
    ///
    /// This method is responsible for dropping all the relation tables related to specifications in the database.
    /// It performs the following steps:
    ///
    /// 1. Drops the `portfolio_instrument` table using the `drop_portfolio_instrument_table` method.
    ///
    /// If the dropping operation is successful, it returns `Ok(())`. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all the relation tables related to specifications are dropped successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error while dropping the relation tables related to specifications.
    ///
    pub async fn drop_all_relation_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_all_relation_tables");

        self.drop_portfolio_instrument_table()
            .await
            .expect("[PostgresUtil]/drop_portfolio_instrument_table: Failed to drop portfolio_instrument table");

        Ok(())
    }
}
