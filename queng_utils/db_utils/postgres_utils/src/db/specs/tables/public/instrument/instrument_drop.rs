use crate::db::all_db_constants::{INSTRUMENT_TABLE, PORTFOLIO_INSTRUMENT_TABLE_INDEX};
use crate::db::common_ddl::{ddl_index, ddl_table};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops the instrument table and its index from the database.
    ///
    /// This method is responsible for dropping the instrument table and its index from the database.
    /// It performs the following steps:
    ///
    /// 1. Drops the `portfolio_instrument_table_index` using the `ddl_index::generate_drop_index_ddl` function.
    /// 2. Drops the `instrument_table` using the `ddl_table::generate_drop_table_ddl` function.
    ///
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the instrument table and its index are dropped successfully.
    /// Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error while dropping the instrument table and its index.
    ///
    pub async fn drop_instrument_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_instrument_table/index");
        let ddl = &ddl_index::generate_drop_index_ddl(PORTFOLIO_INSTRUMENT_TABLE_INDEX);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop instrument table index: {}",
                    e
                )))
            }
        }

        self.dbg_print("drop_instrument_table");
        let ddl = &ddl_table::generate_drop_table_ddl(INSTRUMENT_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop instrument table: {}",
                    e
                )))
            }
        }

        Ok(())
    }
}
