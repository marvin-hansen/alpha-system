use crate::common::all_db_constants::{PORTFOLIO_TABLE, PORTFOLIO_TABLE_INDEX};
use crate::common::common_ddl::{ddl_index, ddl_table};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops the portfolio table, its index, and associated account type from the database.
    ///
    /// This method is responsible for dropping the portfolio table, its index, and associated account type from the database.
    /// It performs the following steps:
    ///
    /// 1. Drops the `portfolio_table_index` using the `ddl_index::generate_drop_index_ddl` function.
    /// 2. Drops the `portfolio_table` using the `ddl_table::generate_drop_table_ddl` function.
    ///
    /// If the dropping operation is successful, it returns `Ok(())`. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the portfolio table, its index, and associated account type are dropped successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error while dropping the portfolio table, its index, or associated account type.
    ///
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

        Ok(())
    }
}
