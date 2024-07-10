use crate::db::specs::Specs;
use crate::prelude::ClickHouseUtilError;

impl Specs {
    /// Asynchronously drops all the necessary tables for the ClickHouse database.
    ///
    /// This method drops all tables required for the ClickHouse database to start fresh. It calls specific drop methods for each table.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of dropping the tables.
    /// If all tables are dropped successfully, it returns `Ok(())`.
    /// If an error occurs during the dropping process, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `ClickHouseUtilError` if any error occurs during the dropping process.
    ///
    pub async fn drop_all_specs_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("/create_all_specs_tables: drop_services_table");
        self.drop_services_table()
            .await
            .expect("[ClickhouseUtil]/create_all_specs_tables: Failed to drop service table");

        Ok(())
    }
}
