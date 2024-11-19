use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
    /// Verifies the existence of all required databases and tables in `ClickHouse`.
    ///
    /// This method internally calls the `verify_metadata_db_exists` and `verify_all_metadata_tables`
    /// methods to check if the metadata database and all required tables exist. It then combines
    /// the results to determine if all necessary databases and tables are present in `ClickHouse`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if all required databases and tables exist, `Ok(false)` if any of them
    /// are missing. If there is an error during the verification process, it returns an
    /// `Err` containing a `ClickHouseUtilError` with details of the error.
    ///
    pub async fn verify_all_db_exists(&self) -> Result<bool, ClickHouseUtilError> {
        self.dbg_print("[verify_all_db]: verify_all_db_exists");
        let metadata_db_exists = self
            .metadata
            .verify_metadata_db_exists()
            .await
            .expect("[verify_db]: Failed to verify if metadata DB exists");

        let metadata_tables_exists = self
            .metadata
            .verify_all_metadata_tables()
            .await
            .expect("[verify_db]: Failed to verify if all metadata tables exists");

        let all_verify = metadata_db_exists && metadata_tables_exists;

        Ok(all_verify)
    }
}
