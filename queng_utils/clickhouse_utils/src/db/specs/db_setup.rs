use crate::db::specs::Specs;
use crate::db::specs::DB_NAME;
use std::error::Error;

impl Specs {
    /// Asynchronously creates the ClickHouse database specified in the `DB_NAME` constant if it does not already exist.
    ///
    /// This method generates a Data Definition Language (DDL) query to create the database
    /// with the name specified in the `DB_NAME` constant.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of creating the database.
    /// If the database is created successfully or already exists, it returns `Ok(())`.
    /// If an error occurs during the creation process, it returns `Err(Box<dyn Error>)` containing the error information.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    ///
    pub async fn create_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    /// Asynchronously verifies if the ClickHouse database specified in the `DB_NAME` constant exists.
    ///
    /// This method executes a query to check if the database with the name specified in the `DB_NAME` constant exists.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of verifying the database existence.
    /// If the database exists, it returns `Ok(true)`.
    /// If the database does not exist, it returns `Ok(false)`.
    /// If an error occurs during the verification process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    ///
    pub async fn verify_specs_db_exists(&self) -> Result<bool, Box<dyn Error>> {
        let exists = self
            .verify_db_exists(DB_NAME)
            .await
            .expect("Failed to verify if metadata DB");

        Ok(exists)
    }
}
