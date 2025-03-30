/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::ClickHouseUtilError;
use crate::db::metadata::{DB_NAME, DB_TABLES, Metadata};

impl Metadata {
    /// Creates all the metadata tables in the database.
    ///
    /// This method creates all the metadata tables in the database.
    /// It performs the following steps:
    ///
    /// 1. Calls the `create_stats_table` method to create the `stats_table`.
    /// 2. Calls the `create_assets_table` method to create the `assets_table`.
    /// 3. Calls the `create_exchanges_table` method to create the `exchanges_table`.
    /// 4. Calls the `create_instruments_table` method to create the `instruments_table`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all the tables were created successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `ClickHouseUtilError` if any of the table creation operations fail.
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
        let tables = DB_TABLES;
        for table_name in tables {
            let query = format!("EXISTS TABLE {DB_NAME}.{table_name};");
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
}
