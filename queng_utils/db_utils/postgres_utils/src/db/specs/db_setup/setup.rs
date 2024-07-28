use std::time::Duration;

use tokio::time;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Sets up the specifications database.
    ///
    /// This method is responsible for setting up the specifications database. It performs the following steps:
    ///
    /// 1. Creates the specifications database using the `create_spec_db` method.
    /// 2. Verifies if the specifications database exists using the `verify_spec_db_exists` method.
    /// 3. Creates the schema for all specifications using the `create_all_spec_schema` method.
    /// 4. Verifies if the schema for all specifications exists using the `verify_all_spec_schema_exists` method.
    /// 5. Creates the tables for all specifications using the `create_all_specs_tables` method.
    /// 6. Verifies if the tables for all specifications exist using the `verify_all_spec_tables_exists` method.
    /// 7. Creates the relation tables for all specifications using the `create_all_specs_relation_tables` method.
    /// 8. Verifies if the relation tables for all specifications exist using the `verify_all_spec_relation_tables_exists` method.
    ///
    /// If any of the above steps fail, an error is returned describing the cause of the failure.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the specifications database is successfully set up.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the setup operations fail.
    ///
    /// # Remarks
    ///
    /// This method is an asynchronous function and should be awaited in an asynchronous context.
    /// It is essential to handle errors appropriately when using this method to set up the specifications database.
    ///
    /// # Safety
    ///
    /// This method assumes the underlying database creation, schema creation, and table creation mechanisms are correctly implemented.
    /// Ensure that the setup operations are intended and the consequences of setting up the specifications database are understood before calling this method.
    ///
    /// # Panics
    ///
    /// This method does not panic under normal circumstances. Any unexpected behavior should result in an `Err` variant being returned.
    ///
    /// # Aborts
    ///
    /// This method does not abort the program. It provides a controlled way to handle setting up the specifications database.
    ///
    pub async fn setup_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("setup_spec_db");

        match self.create_spec_db().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create specs DB: {}",
                    e
                )))
            }
        }

        match self.verify_spec_db_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if specs DB exists: {}",
                    e
                )))
            }
        };

        match self.create_all_spec_schema().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create all specs DB schema: {}",
                    e
                )))
            }
        }
        // Schema propagation somehow takes a moment; just wait a second.
        time::sleep(Duration::from_secs(1)).await;

        match self.verify_all_spec_schema_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if all specs DB schema exists: {}",
                    e
                )))
            }
        }

        match self.create_all_specs_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create all specs tables: {}",
                    e
                )))
            }
        };

        match self.verify_all_spec_tables_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if all specs tables exists: {}",
                    e
                )))
            }
        }

        match self.create_all_specs_relation_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to create all specs relation tables: {}",
                    e
                )))
            }
        }

        match self.verify_all_spec_relation_tables_exists().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to verify if all specs relation tables exists: {}",
                    e
                )))
            }
        }

        Ok(())
    }
}
