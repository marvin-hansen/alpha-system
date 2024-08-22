use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    /// Asynchronously sets up all tables in the ClickHouse database.
    ///
    /// This method triggers the setup process for all tables in the ClickHouse database.
    /// Each table's setup is performed asynchronously.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of setting up all tables.
    /// If all tables are set up successfully, it returns `Ok(())`.
    /// If an error occurs during the setup process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    ///
    pub async fn setup_all_tables(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[setup_all_tables]: create all metadata tables");
        self.metadata
            .create_all_metadata_tables()
            .await
            .expect("[setup_db]: Failed create all metadata tables");

        Ok(())
    }

    /// Asynchronously tears down all tables in the ClickHouse database.
    ///
    /// This method triggers the teardown process for all tables in the ClickHouse database.
    /// Each table's teardown is performed asynchronously.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of tearing down all tables.
    /// If all tables are torn down successfully, it returns `Ok(())`.
    /// If an error occurs during the teardown process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    ///
    pub async fn drop_all_tables(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[drop_all_tables]: Create all metadata tables");
        self.metadata
            .drop_all_metadata_tables()
            .await
            .expect("[teardown_db]: Failed to drop all metadata tables");

        Ok(())
    }
}
