/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::common_ddl::ddl_db;
use crate::db::metadata::{Metadata, DB_NAME};
use crate::db::utils;
use crate::error::ClickHouseQueryError;

impl Metadata {
    pub(crate) fn generate_drop_table_ddl(&self, table_name: &str) -> String {
        ddl_db::generate_drop_table_ddl(table_name, DB_NAME)
    }

    /// Asynchronously executes a query in the metadata database.
    ///
    /// This method executes the specified query in the metadata database using the `ClickHouse` client.
    ///
    /// # Arguments
    ///
    /// - `query`: A string slice containing the query to be executed.
    ///
    /// # Returns
    ///
    /// Returns `Result<(), ClickHouseQueryError>` indicating the success or failure of the query execution.
    /// If the query execution is successful, it returns `Ok(())`.
    /// If an error occurs during the query execution, it returns an `Err(ClickHouseQueryError)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `ClickHouseQueryError` trait.
    ///
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), ClickHouseQueryError> {
        utils::execute_query(&self.client, query)
            .await
            .expect("Failed to query metadata DB");

        Ok(())
    }

    /// Asynchronously verifies the existence of a table in the metadata.
    ///
    /// This method checks if the specified table exists in the metadata database.
    ///
    /// # Arguments
    ///
    /// - `table_name`: A string slice containing the name of the table to verify.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a boolean value indicating whether the table exists.
    /// - `Ok(true)` if the table exists.
    /// - `Ok(false)` if the table does not exist.
    ///
    /// If an error occurs during the verification process, it returns an `Err(ClickHouseQueryError)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `ClickHouseQueryError` trait.
    ///
    pub(crate) async fn verify_table_exists(
        &self,
        query: &str,
    ) -> Result<bool, ClickHouseQueryError> {
        let res = utils::verify_table_exists(&self.client, query)
            .await
            .expect("Failed to verify that table exists in metadata DB");

        Ok(res)
    }

    /// Asynchronously verifies the existence of a database in the metadata.
    ///
    /// This method checks if the specified database exists in the metadata database.
    ///
    /// # Arguments
    ///
    /// - `db_name`: A string slice containing the name of the database to verify.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a boolean value indicating whether the database exists.
    /// - `Ok(true)` if the database exists.
    /// - `Ok(false)` if the database does not exist.
    ///
    /// If an error occurs during the verification process, it returns an `Err(ClickHouseQueryError)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `ClickHouseQueryError` trait.
    ///
    pub(crate) async fn verify_db_exists(
        &self,
        db_name: &str,
    ) -> Result<bool, ClickHouseQueryError> {
        let res = utils::verify_db_exists(&self.client, db_name)
            .await
            .expect("Failed to verify that table exists in metadata DB");

        Ok(res)
    }

    /// Asynchronously counts the number of rows in the specified table.
    ///
    /// This method queries the metadata database to retrieve the total number of rows present in the
    /// specified table. It returns the count as a `u64` value representing the number of rows in the table.
    ///
    /// # Arguments
    ///
    /// - `table_name`: A string slice containing the name of the table to count rows from.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the count of rows as a `u64` if the query is successful.
    ///
    /// If an error occurs during the count operation, it returns an `Err(ClickHouseQueryError)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `ClickHouseQueryError` trait.
    ///
    pub(crate) async fn count_rows(&self, table_name: &str) -> Result<u64, ClickHouseQueryError> {
        let res = utils::count_rows(&self.client, table_name)
            .await
            .expect("Failed to count table rows in metadata DB");

        Ok(res)
    }
}
