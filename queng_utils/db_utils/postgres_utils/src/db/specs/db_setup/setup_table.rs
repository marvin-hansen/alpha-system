use crate::db::all_db_constants::{
    DEFAULT_SCHEMA, INSTRUMENT_TABLE, PORTFOLIO_TABLE, SERVICE_TABLE, SYSTEM_SCHEMA,
};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Asynchronously creates all the necessary tables for the Surreal database.
    ///
    pub(crate) async fn create_all_specs_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("/create_all_specs_tables: create_service_table");
        self.create_service_table()
            .await
            .expect("[PostgresUtil]/create_all_specs_tables: Failed to create service table");

        self.dbg_print("/create_all_specs_tables: create_portfolio_table");
        self.create_portfolio_table()
            .await
            .expect("[PostgresUtil]/create_portfolio_table: Failed to create portfolio table");

        self.dbg_print("/create_all_specs_tables: create_instrument_table");
        self.create_instrument_table()
            .await
            .expect("[PostgresUtil]/create_instrument_table: Failed to create instrument table");

        Ok(())
    }

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
                    e.to_string()
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
                    e.to_string()
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
                    e.to_string()
                )))
            }
        };

        Ok(true)
    }
}
