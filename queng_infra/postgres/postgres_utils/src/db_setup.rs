use crate::prelude::PostgresUtilError;
use crate::PostgresUtil;

impl PostgresUtil {
    /// Sets up all databases.
    ///
    /// This method sets up all databases by performing the following steps:
    ///
    /// 1. Sets up the specifications database using the `setup_spec_db` method.
    ///
    /// If any of the above steps fail, an error is returned describing the cause of the failure.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all databases are successfully set up.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the setup operations fail.
    ///
    pub async fn setup_all_db(&self) -> Result<(), PostgresUtilError> {
        let conn = &mut self.pool.get().unwrap();

        self.dbg_print("[setup_all_db]: setup SMDB DB schema");
        match pg_smdb::run_smdb_db_migration(conn) {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        }

        self.dbg_print("[setup_all_db]: setup CMDB DB schema");
        match pg_cmdb::run_cmdb_db_migration(conn) {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        }

        Ok(())
    }
}
