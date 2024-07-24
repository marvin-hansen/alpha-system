use crate::common::all_db_constants::DB_NAME;
use crate::common::common_ddl::ddl_db;
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
        match self.teardown_spec_db(true).await {
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
}
