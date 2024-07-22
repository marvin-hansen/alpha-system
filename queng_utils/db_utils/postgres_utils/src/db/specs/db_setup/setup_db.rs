use crate::db::all_db_constants::DB_NAME;
use crate::db::common_ddl::{ddl_db, ddl_verify};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Creates the specifications database.
    ///
    /// This method creates the specifications database by performing the following steps:
    ///
    /// 1. Drops the existing specifications database using the `drop_db` method.
    /// 2. Generates the DDL (Data Definition Language) for creating the specifications database using the `ddl_db::generate_create_db_ddl` function.
    /// 3. Executes the generated DDL using the `execute_query` method.
    ///
    /// If any of the above steps fail, an error is returned describing the cause of the failure.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the specifications database is successfully created.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the creation operations fail.
    ///
    pub(crate) async fn create_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_spec_db");

        self.dbg_print("drop_spec_db");
        match self.drop_db(DB_NAME).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Error: Failed to drop specs DB: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_spec_db");
        let create_ddl = &ddl_db::generate_create_db_ddl(DB_NAME);
        match self.execute_query(create_ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(format!(
                "Failed to create specs DB: {}",
                e
            ))),
        }
    }

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
