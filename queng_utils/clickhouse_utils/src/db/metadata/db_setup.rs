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
    pub async fn verify_all_metadata_tables(&self) -> Result<bool, ClickHouseUtilError> {
        self.verify_metadata_tables_created().await
    }

    pub(crate) async fn verify_metadata_tables_created(&self) -> Result<bool, ClickHouseUtilError> {
        let tables = self.metadata_tables();
        for table in tables {
            let query = self.generate_table_exists_query(table);
            match self.verify_exists(&query).await {
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
    pub(crate) async fn create_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("create_metadata_db");
        let ddl = format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to create metadata DB");

        Ok(())
    }
}

impl Metadata {
    /// Creates all the metadata tables in the metadata database.
    ///
    /// This method creates all the metadata tables in the metadata database using the `create_all_metadata_tables` method.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - Returns `Ok(())` if the tables are created successfully, or an `Err` containing the error if it fails.
    ///
    pub async fn create_all_metadata_tables(&self) -> Result<(), ClickHouseUtilError> {
        //
        self.dbg_print("/create_all_metadata_tables: create_stats_table");
        self.create_stats_table()
            .await
            .expect("[ClickhouseUtil]/create_all_metadata_tables: Failed to create stats table");

        self.dbg_print("create_assets_table");
        self.create_assets_table()
            .await
            .expect("[ClickhouseUtil]/create_all_metadata_tables: Failed to create asset table");

        self.dbg_print("/create_all_metadata_tables: create_exchanges_table");
        self.create_exchanges_table().await.expect(
            "[ClickhouseUtil]/create_all_metadata_tables: Failed to create exchanges table",
        );

        self.dbg_print("/create_all_metadata_tables: create_instruments_table");
        self.create_instruments_table().await.expect(
            "[ClickhouseUtil]/create_all_metadata_tables: Failed to create instruments table",
        );

        Ok(())
    }
}
