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
        self.dbg_print("[setup_all_db]: setup_spec_db");
        self.specs
            .setup_spec_db()
            .await
            .expect("[setup_db]: Failed to setup specs DB");

        Ok(())
    }
}
