use crate::db::all_db_constants::{
    PORTFOLIO_TABLE, PORTFOLIO_TABLE_ACCOUNT_TYPE, PORTFOLIO_TABLE_INDEX,
};
use crate::db::common_ddl::{ddl_index, ddl_table, ddl_type};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_portfolio_table/index");
        let ddl = &ddl_index::generate_drop_index_ddl(PORTFOLIO_TABLE_INDEX);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop portfolio table index: {}",
                    e
                )))
            }
        }

        self.dbg_print("drop_portfolio_table");
        let ddl = &ddl_table::generate_drop_table_ddl(PORTFOLIO_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop portfolio table: {}",
                    e
                )))
            }
        }

        self.dbg_print("drop_portfolio_table/account_type");
        let ddl = &ddl_type::generate_drop_type_ddl(PORTFOLIO_TABLE_ACCOUNT_TYPE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop portfolio table index: {}",
                    e
                )))
            }
        }

        Ok(())
    }
}
