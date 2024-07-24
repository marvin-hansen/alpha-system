use crate::common::all_db_constants::DB_NAME;
use crate::common::common_ddl::ddl_verify;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Verifies if the specifications database exists.
    ///
    /// This method checks if the specifications database exists in the database server.
    /// It performs the following steps:
    ///
    /// 1. Generates the DDL (Data Definition Language) for verifying the existence of the specifications database using the `ddl_verify::generate_verify_db_ddl` function.
    /// 2. Executes the generated DDL using the `execute_verify_query` method.
    ///
    /// If the specifications database exists, it returns `Ok(true)`. Otherwise, it returns `Ok(false)`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the specifications database exists, `Ok(false)` if it does not.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error in verifying the existence of the specifications database.
    ///
    pub(crate) async fn verify_spec_db_exists(&self) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_spec_db_exists");
        let verify_ddl = &ddl_verify::generate_verify_db_ddl(DB_NAME);
        match self.execute_verify_query(verify_ddl).await {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to verify if specs DB exists: {}",
                e
            ))),
        }
    }
}
