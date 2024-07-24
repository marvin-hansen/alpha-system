use crate::common::all_db_constants::{
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

        match self.verify_table_exists(SYSTEM_SCHEMA, SERVICE_TABLE).await {
            Ok(res) => {
                if !res {
                    return Err(PostgresUtilError::new(
                        "Failed to verify service table: Service table does not exist ".to_string(),
                    ));
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify service schema: {}",
                    e
                )))
            }
        };

        self.dbg_print("/create_all_specs_tables: create_portfolio_table");
        self.create_portfolio_table()
            .await
            .expect("[PostgresUtil]/create_portfolio_table: Failed to create portfolio table");

        match self
            .verify_table_exists(DEFAULT_SCHEMA, PORTFOLIO_TABLE)
            .await
        {
            Ok(res) => {
                if !res {
                    return Err(PostgresUtilError::new(
                        "Failed to verify portfolio table: Portfolio table does not exist "
                            .to_string(),
                    ));
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify portfolio schema: {}",
                    e
                )))
            }
        }

        self.dbg_print("/create_all_specs_tables: create_instrument_table");
        self.create_instrument_table()
            .await
            .expect("[PostgresUtil]/create_instrument_table: Failed to create instrument table");

        match self
            .verify_table_exists(DEFAULT_SCHEMA, INSTRUMENT_TABLE)
            .await
        {
            Ok(res) => {
                if !res {
                    return Err(PostgresUtilError::new(
                        "Failed to verify instrument table: Instrument table does not exist "
                            .to_string(),
                    ));
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify instrument schema: {}",
                    e
                )))
            }
        }

        Ok(())
    }
}
