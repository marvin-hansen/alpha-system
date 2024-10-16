use crate::prelude::PostgresUtil;
use crate::prelude::PostgresUtilError;

impl PostgresUtil {
    /// Tears down all databases.
    ///
    /// This method tears down all databases by delegating the teardown operation to the `teardown_spec_db` method of the `Specs` struct.
    ///
    /// # Arguments
    ///
    /// * `drop`: A boolean flag indicating whether to drop the databases during teardown.
    ///   If `true`, the databases will be dropped. If `false`, the databases will be left intact.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all databases are successfully torn down.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the teardown operations fail.
    ///
    pub async fn teardown_all_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("teardown_all_db");

        let conn = &mut self.pool.get().unwrap();

        let result = pg_cmdb::revert_cmdb_db_migration(conn);
        //dbg!(&result);
        assert!(result.is_ok());
        let result = pg_imdb::revert_imdb_db_migration(conn);
        //dbg!(&result);
        assert!(result.is_ok());
        let result = pg_smdb::revert_smdb_db_migration(conn);
        //dbg!(&result);
        assert!(result.is_ok());
        let result = pg_mddb::revert_mddb_db_migration(conn);
        //dbg!(&result);
        assert!(result.is_ok());

        Ok(())
    }
}
