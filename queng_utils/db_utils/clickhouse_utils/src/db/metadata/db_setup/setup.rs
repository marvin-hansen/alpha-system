use crate::db::metadata::Metadata;
use crate::prelude::ClickHouseUtilError;

impl Metadata {
    /// Sets up the metadata database for ClickHouse.
    ///
    /// This method initializes the metadata database by performing the following steps:
    /// 1. Calls the `create_metadata_db` method to create the metadata database.
    /// 2. Verifies the existence of the metadata database by calling `verify_metadata_db_exists`.
    /// 3. Creates all necessary tables in the metadata database using `create_all_metadata_tables`.
    /// 4. Verifies the existence of all required tables with `verify_all_metadata_tables`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success of the metadata database setup.
    /// - If successful, it returns `Ok(())`.
    /// - If an error occurs, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `ClickHouseUtilError` trait.
    ///
    pub async fn setup_metadata_db(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("setup_metadata_db");
        match self.create_metadata_db().await {
            Ok(()) => (),
            Err(e) => return Err(e),
        }

        self.dbg_print("verify_metadata_db_exists");
        match self.verify_metadata_db_exists().await {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        self.dbg_print("create_all_metadata_tables");
        match self.create_all_metadata_tables().await {
            Ok(()) => (),
            Err(e) => return Err(e),
        };

        self.dbg_print("verify_all_metadata_tables");
        match self.verify_all_metadata_tables().await {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
