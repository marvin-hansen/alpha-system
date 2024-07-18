use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_all_relation_tables(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_all_relation_tables");

        self.drop_portfolio_instrument_table()
            .await
            .expect("[PostgresUtil]/drop_portfolio_instrument_table: Failed to drop portfolio_instrument table");

        Ok(())
    }
}
