use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops the specifications database and its associated tables and schemas.
    ///
    /// This method is responsible for dropping the specifications database and its associated tables and schemas.
    /// It performs the following steps:
    ///
    /// - Drops all the relation tables.
    /// - Drops all the specifications tables.
    /// - Drops all the specifications schema.
    ///
    /// # Arguments
    ///
    /// * `drop` - A boolean flag indicating whether to drop the specifications database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the following outcomes:
    ///
    /// - `Ok(())` if the database and its associated tables and schemas are dropped successfully.
    /// - `Err` variant containing a `PostgresUtilError` if there is an error while dropping the database or its associated tables and schemas.
    ///
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is a failure in dropping the specifications database or its associated tables and schemas.
    ///
    /// # Safety
    ///
    /// This method assumes the database to be dropped is correctly specified in the `DB_NAME` constant and that the underlying dropping mechanism is implemented correctly.
    /// Ensure that the dropping operation is intended and the consequences of dropping the database and its associated tables and schemas are understood before calling this method.
    ///
    /// # Panics
    ///
    /// This method does not panic under normal circumstances. Any unexpected behavior should result in an `Err` variant being returned.
    ///
    /// # Aborts
    ///
    /// This method does not abort the program. It provides a controlled way to handle dropping the specifications database and its associated tables and schemas.
    ///
    /// # Remarks
    ///
    /// This method is an asynchronous function and should be awaited in an asynchronous context.
    /// It is essential to handle errors appropriately when using this method to drop the specifications tables.
    ///
    pub async fn teardown_spec_db(&self, drop: bool) -> Result<(), PostgresUtilError> {
        self.dbg_print("teardown_spec_db");

        self.dbg_print("[teardown_spec_db]: drop_all_relation_tables");
        match self.drop_all_relation_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop all relation tables: {}",
                    e
                )))
            }
        }

        self.dbg_print("[teardown_spec_db]: drop_all_specs_tables");
        match self.drop_all_specs_tables().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop all specs tables: {}",
                    e
                )))
            }
        }

        self.dbg_print("[teardown_spec_db]: drop_all_specs_schema");
        match self.drop_all_specs_schema().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop all specs schema: {}",
                    e
                )))
            }
        }

        if drop {
            self.dbg_print("[teardown_spec_db]: drop_spec_db");
            match self.drop_spec_db().await {
                Ok(_) => (),
                Err(e) => {
                    return Err(PostgresUtilError::new(format!(
                        "Error: Failed to drop specs DB: {}",
                        e
                    )))
                }
            }
        }

        Ok(())
    }
}
