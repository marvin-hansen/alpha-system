use crate::prelude::PostgresUtil;
use crate::prelude::PostgresUtilError;

impl PostgresUtil {
    /// Verifies all databases associated with the PostgresUtil object.
    ///
    /// This function verifies the specs database by calling the `verify_spec_db` method on the `specs` object.
    /// It returns `Ok(true)` if all databases are successfully verified, and `Err(PostgresUtilError)` if any verification fails.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether all databases have been verified.
    /// If successful, it returns `Ok(true)`.
    /// If an error occurs, it returns `Err(PostgresUtilError)`.
    ///
    ///
    pub async fn verify_all_db(&self) -> Result<bool, PostgresUtilError> {
        let conn = &mut self.pool.get().unwrap();

        self.dbg_print("[verify_all_db]: verify SMDB DB schema");
        match pg_smdb::check_smdb_db_migration(conn) {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        }

        self.dbg_print("[verify_all_db]: verify CMDB DB schema");
        match pg_cmdb::check_cmdb_db_migration(conn) {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        }

        Ok(true)
    }
}
