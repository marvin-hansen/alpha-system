use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Asynchronously drops all the necessary tables for the Surreal database.
    ///
    /// This method drops all tables required for the Surreal database to start fresh.
    /// It calls specific drop methods for each table.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of dropping the tables.
    /// If all tables are dropped successfully, it returns `Ok(())`.
    /// If an error occurs during the dropping process, it returns `Err(SurrealUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `SurrealUtilError` if any error occurs during the dropping process.
    ///
    pub async fn drop_all_specs_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("/drop_all_specs_tables: drop_portfolio_table");
        self.drop_portfolio_table()
            .await
            .expect("[SurrealUtil]/drop_portfolio_table: Failed to drop portfolio table");

        self.dbg_print("/drop_all_specs_tables: drop_services_table");
        self.drop_service_table()
            .await
            .expect("[SurrealUtil]/drop_all_specs_tables: Failed to drop service table");

        Ok(())
    }
}
