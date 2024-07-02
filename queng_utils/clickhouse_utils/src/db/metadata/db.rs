use crate::db::metadata::{Metadata, DB_NAME, DB_TABLES};
use crate::types::error::ClickHouseUtilError;
use std::error::Error;

impl Metadata {
    pub(crate) fn metadata_tables(&self) -> [&'static str; 4] {
        DB_TABLES
    }
}

impl Metadata {
    /// Verifies that all metadata tables have been created.
    ///
    /// This method performs the following steps:
    ///
    /// 1. Retrieves the list of metadata tables.
    /// 2. For each table, it generates a query to check if the table exists.
    /// 3. Executes the query and checks if the table exists.
    /// 4. If any table does not exist, it returns `Ok(false)`.
    /// 5. If all tables exist, it returns `Ok(true)`.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if all metadata tables have been created.
    /// - `Ok(false)` if any metadata table does not exist.
    /// - `Err(ClickHouseUtilError)` if an error occurs during the verification process.
    ///
    pub(crate) async fn verify_metadata_tables_created(&self) -> Result<bool, ClickHouseUtilError> {
        let tables = self.metadata_tables();
        for table in tables {
            let query = self.generate_table_exists_query(table);
            match self.verify_table_exists(&query).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(ClickHouseUtilError::from(e.to_string())),
            }
        }

        Ok(true)
    }
    fn generate_table_exists_query(&self, table_name: &str) -> String {
        format!("EXISTS TABLE {DB_NAME}.{table_name};")
    }
}

impl Metadata {
    ///
    /// This method creates the metadata database if it does not already exist.
    ///
    pub(crate) async fn create_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.create_metadata_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to create metadata DB");

        Ok(())
    }

    fn create_metadata_ddl(&self) -> String {
        format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}")
    }
}

impl Metadata {
    ///
    /// This method drops the metadata database if it exists.
    ///
    pub(crate) async fn drop_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.drop_metadata_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }
    fn drop_metadata_ddl(&self) -> String {
        format!("DROP DATABASE IF EXISTS {DB_NAME}")
    }
}
