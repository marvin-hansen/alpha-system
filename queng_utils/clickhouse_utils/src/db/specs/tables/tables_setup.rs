use crate::db::specs::{Specs, DB_NAME, DB_TABLES};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    /// Asynchronously creates all the necessary tables for the ClickHouse database.
    ///
    pub async fn create_all_specs_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("/create_all_specs_tables: create_service_table");
        self.create_service_table()
            .await
            .expect("[ClickhouseUtil]/create_all_specs_tables: Failed to create service table");

        Ok(())
    }

    /// Asynchronously verifies if all the necessary tables for the ClickHouse database are created.
    ///
    /// This method iterates through the list of tables defined in the `DB_TABLES` constant and checks if each table exists in the database.
    /// It does this by executing a query to check if the table exists using the `verify_table_exists` method.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of verifying the tables.
    /// If all tables exist, it returns `Ok(true)`.
    /// If any table does not exist, it returns `Ok(false)`.
    /// If an error occurs during the verification process, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `ClickHouseUtilError`.
    ///
    pub async fn verify_all_specs_tables(&self) -> Result<bool, ClickHouseUtilError> {
        self.verify_specs_tables_created().await
    }

    async fn verify_specs_tables_created(&self) -> Result<bool, ClickHouseUtilError> {
        let tables = DB_TABLES;
        for table_name in tables {
            let query = format!("EXISTS TABLE {DB_NAME}.{table_name};");
            match self.verify_table_exists(&query).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(ClickHouseUtilError::from(e.to_string())),
            }
        }

        Ok(true)
    }
}
