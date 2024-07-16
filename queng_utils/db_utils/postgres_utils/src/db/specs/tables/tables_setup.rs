use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Asynchronously creates all the necessary tables for the Surreal database.
    ///
    pub async fn create_all_specs_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("/create_all_specs_tables: create_portfolio_table");
        self.create_portfolio_table()
            .await
            .expect("[SurrealUtil]/create_portfolio_table: Failed to create portfolio table");

        self.dbg_print("/create_all_specs_tables: create_service_table");
        self.create_service_table()
            .await
            .expect("[SurrealUtil]/create_all_specs_tables: Failed to create service table");

        Ok(())
    }
}
