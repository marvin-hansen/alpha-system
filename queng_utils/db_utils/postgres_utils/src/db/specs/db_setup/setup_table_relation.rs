use crate::db::all_db_constants::{DEFAULT_SCHEMA, PORTFOLIO_INSTRUMENT_TABLE};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub(crate) async fn create_all_specs_relation_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("/create_all_specs_relation_tables: create_portfolio_instrument_table");
        self.create_portfolio_instrument_table()
            .await
            .expect("[PostgresUtil]/create_all_specs_relation_tables: Failed to create portfolio_instrument table");

        Ok(())
    }

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
                e.to_string()
            ))),
        }
    }
}
