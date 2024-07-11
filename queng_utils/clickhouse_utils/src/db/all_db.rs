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
            .create_metadata_db()
            .await
            .expect("[setup_db]: Failed to create metadata DB");

        self.dbg_print("[setup_all_db]: create_spec_db");
        self.specs
            .create_spec_db()
            .await
            .expect("[setup_db]: Failed to create specs DB");

        Ok(())
    }

    /// Asynchronously checks if the metadata and specs databases exist.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of checking if the databases exist.
    /// If the databases exist, it returns `Ok(true)`.
    /// If the databases do not exist, it returns `Ok(false)`.
    /// If an error occurs during the check process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    pub async fn verify_all_db_exists(&self) -> Result<bool, Box<dyn Error>> {
        self.dbg_print("[setup_all_db]: verify_metadata_db_exists");
        let metadata_db = self
            .metadata
            .verify_metadata_db_exists()
            .await
            .expect("[setup_db]: Failed to verify if metadata DB exists");

        let specs_db = self
            .specs
            .verify_specs_db_exists()
            .await
            .expect("[setup_db]: Failed to verify if specs DB exists");

        let all_db = metadata_db && specs_db;

        Ok(all_db)
    }

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
    pub async fn teardown_all_db(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[teardown_all_db]: drop_metadata_db");
        self.metadata
            .drop_metadata_db()
            .await
            .expect("[teardown_db]: Failed to drop metadata DB");

        self.dbg_print("[teardown_all_db]: drop_spec_db");
        self.specs
            .drop_spec_db()
            .await
            .expect("[teardown_db]: Failed to drop specs DB");

        Ok(())
    }
}
