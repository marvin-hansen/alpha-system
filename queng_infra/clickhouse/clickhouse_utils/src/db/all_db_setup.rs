use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    /// Asynchronously sets up the ClickHouse databases for the `AllDb` struct.
    ///
    /// This method creates the metadata database and specs database if they do not already exist.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of setting up the databases.
    /// If the databases are set up successfully, it returns `Ok(())`.
    /// If an error occurs during the setup process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    ///
    pub async fn setup_all_db(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[setup_all_db]: create_metadata_db");
        self.metadata
            .setup_metadata_db()
            .await
            .expect("[setup_db]: Failed to create metadata DB");

        Ok(())
    }
}
