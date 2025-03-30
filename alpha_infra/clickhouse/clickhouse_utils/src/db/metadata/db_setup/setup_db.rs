/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::{DB_NAME, Metadata};
use crate::error::ClickHouseUtilError;

impl Metadata {
    /// Creates the metadata database if it does not already exist.
    ///
    /// This method generates a `CREATE DATABASE` query with the name of the metadata database and executes it using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` if the database is created successfully, or an `Err` containing the error if it fails.
    ///
    /// # Errors
    ///
    /// Returns a `Box<dyn Error>` if there is an error executing the `CREATE DATABASE` query.
    ///
    pub async fn create_metadata_db(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("create_metadata_db");
        let ddl = format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to create metadata DB");

        Ok(())
    }
    pub async fn verify_metadata_db_exists(&self) -> Result<bool, ClickHouseUtilError> {
        let exists = self
            .verify_db_exists(DB_NAME)
            .await
            .expect("Failed to verify if metadata DB");

        Ok(exists)
    }
}
