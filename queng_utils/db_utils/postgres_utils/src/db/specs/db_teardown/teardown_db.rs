use crate::common::all_db_constants::DB_NAME;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops the specifications database.
    ///
    /// This method is responsible for dropping the specifications database. It performs the following steps:
    ///
    /// 1. Prints a debug message indicating the intention to drop the specifications database.
    /// 2. Attempts to drop the database with the name specified in the constant `DB_NAME`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the following outcomes:
    /// - `Ok(())` if the database is dropped successfully.
    /// - `Err` variant containing a `PostgresUtilError` if there is an error while dropping the database.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is a failure in dropping the specifications database.
    ///
    pub async fn drop_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_spec_db");
        match self.drop_db(DB_NAME).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop specs DB: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
