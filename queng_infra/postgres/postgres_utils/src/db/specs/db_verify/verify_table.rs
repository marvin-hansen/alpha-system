use crate::common::all_db_constants::{
    DEFAULT_SCHEMA, INSTRUMENT_TABLE, PORTFOLIO_TABLE, SERVICE_TABLE, SYSTEM_SCHEMA,
};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Verifies if all specification tables exist.
    ///
    /// This method verifies if all specification tables exist in the database. It performs the following steps:
    ///
    /// 1. Verifies if the `service` table exists.
    /// 2. Verifies if the `portfolio` table exists.
    /// 3. Verifies if the `instrument` table exists.
    ///
    /// If all tables exist, it returns `Ok(true)`. Otherwise, it returns `Ok(false)`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if all specification tables exist, `Ok(false)` if any table does not exist.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error in verifying the existence of the tables.
    ///
    pub(crate) async fn verify_all_spec_tables_exists(&self) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_all_spec_tables_exists");

        match self.verify_table_exists(SYSTEM_SCHEMA, SERVICE_TABLE).await {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify service schema: {}",
                    e
                )))
            }
        };

        match self
            .verify_table_exists(DEFAULT_SCHEMA, PORTFOLIO_TABLE)
            .await
        {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify portfolio schema: {}",
                    e
                )))
            }
        };

        match self
            .verify_table_exists(DEFAULT_SCHEMA, INSTRUMENT_TABLE)
            .await
        {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify instrument schema: {}",
                    e
                )))
            }
        };

        Ok(true)
    }
}
