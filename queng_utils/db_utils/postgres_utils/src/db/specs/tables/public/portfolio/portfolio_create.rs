use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Creates the portfolio table, account type, and indexes in the database.
    ///
    /// This method is responsible for creating the portfolio table, its account type, and indexes in the database.
    /// It performs the following steps:
    ///
    /// * Generates and executes the DDL for creating the portfolio table using `generate_portfolio_table_ddl`.
    /// * Generates and executes the DDL for creating the portfolio table indexes using `generate_portfolio_table_index_ddl`.
    ///
    /// If the creation operation is successful, it returns `Ok(())`. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the portfolio table, account type, and indexes are created successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error during the creation of the portfolio table, account type, or indexes.
    ///
    ///
    /// # Remarks
    ///
    /// This method is asynchronous and should be awaited in an asynchronous context.
    /// It is important to handle errors properly when creating the portfolio table, account type, and indexes.
    ///
    /// # Safety
    ///
    /// This method assumes the correctness of the underlying table and index creation mechanism.
    /// Ensure that the table and index creation operations are intended and
    /// the implications of creating the portfolio table, account type, and indexes are understood
    /// before calling this method.
    ///
    /// # Panics
    ///
    /// This method does not panic under normal circumstances.
    /// Any unexpected behavior should result in an `Err` variant being returned.
    ///
    /// # Aborts
    ///
    /// This method does not abort the program. It provides a controlled way to create the
    /// portfolio table, account type, and indexes.
    ///
    pub async fn create_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_portfolio_table");

        self.dbg_print("create_portfolio_table");
        let ddl = self.generate_portfolio_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create portfolio table: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_portfolio_table/index");
        let index_ddl = self.generate_portfolio_table_index_ddl();
        match self.execute_query(&index_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create portfolio table indexes: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
