use crate::prelude::PostgresUtilError;
use crate::PostgresUtil;

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
    pub async fn teardown_all_db(&self, drop: bool) -> Result<(), PostgresUtilError> {
        self.dbg_print("teardown_all_db");

        self.dbg_print("[teardown_all_db]: teardown_spec_db");
        self.specs
            .teardown_spec_db(drop)
            .await
            .expect("Failed to drop specs DB");
        Ok(())
    }

    /// Drops all databases.
    ///
    /// This method drops all databases by delegating the drop operation to the `drop_spec_db` method of the `Specs` struct.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all databases are successfully dropped.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if any of the drop operations fail.
    ///
    pub async fn drop_all_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_all_db");

        self.dbg_print("[drop_all_db]: drop_spec_db");
        self.specs
            .drop_spec_db()
            .await
            .expect("Failed to drop spec DB");

        Ok(())
    }
}
