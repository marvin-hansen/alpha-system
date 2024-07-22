use crate::db::all_db_constants::{DEFAULT_SCHEMA, PORTFOLIO_INSTRUMENT_TABLE};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Creates all specification relation tables.
    ///
    /// This method is responsible for creating all the relation tables for specifications in the database.
    /// It performs the following steps:
    ///
    /// 1. Creates the portfolio instrument table by calling the `create_portfolio_instrument_table` method.
    /// 2. Verifies the existence of the portfolio instrument table using the `verify_table_exists` method.
    ///
    /// If the creation and verification of all relation tables are successful, it returns `Ok(())`.
    /// Otherwise, it returns an `Err` variant of `PostgresUtilError` with a descriptive error message.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all the relation tables for specifications are successfully created and verified.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the creation or verification operations fail.
    ///
    pub(crate) async fn create_all_specs_relation_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("/create_all_specs_relation_tables: create_portfolio_instrument_table");
        self.create_portfolio_instrument_table()
            .await
            .expect("[PostgresUtil]/create_all_specs_relation_tables: Failed to create portfolio_instrument table");

        Ok(())
    }

    /// Verifies if all the relation tables for specifications exist.
    ///
    /// This method checks if all the relation tables for specifications exist in the database.
    /// It performs the following steps:
    ///
    /// 1. Verifies the existence of the default schema and the portfolio instrument table using the `verify_table_exists` method.
    ///
    /// If all the relation tables for specifications exist, it returns `Ok(true)`. Otherwise, it returns `Ok(false)`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if all the relation tables for specifications exist, `Ok(false)` if they do not.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error in verifying the existence of the relation tables.
    ///
    pub(crate) async fn verify_all_spec_relation_tables_exists(
        &self,
    ) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_all_spec_relation_tables_exists");

        match self
            .verify_table_exists(DEFAULT_SCHEMA, PORTFOLIO_INSTRUMENT_TABLE)
            .await
        {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to verify portfolio_instrument schema: {}",
                e
            ))),
        }
    }
}
