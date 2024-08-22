use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
    /// Asynchronously drops the metadata and specs databases.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of dropping the databases.
    /// If the databases are dropped successfully, it returns `Ok(())`.
    /// If an error occurs during the drop process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    pub async fn teardown_all_db(&self, drop_db: bool) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("[teardown_all_db]: drop_metadata_db");
        self.metadata
            .teardown_metadata_db(drop_db)
            .await
            .expect("[teardown_db]: Failed to drop metadata DB");

        Ok(())
    }

    pub async fn drop_all_db(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("[drop_all_db]: drop_metadata_db");
        self.metadata
            .drop_metadata_db()
            .await
            .expect("[drop_db]: Failed to drop metadata DB");
        Ok(())
    }
}
