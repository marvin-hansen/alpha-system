/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::ClickHouseUtilError;
use crate::db::metadata::{DB_NAME, EXCHANGES_TABLE, Metadata};
use common_metadata::MetaExchange;

impl Metadata {
    /// Imports a list of exchanges metadata into the metadata database.
    ///
    /// This method takes a vector of `Exchange` objects and imports their metadata into the metadata database.
    /// It generates an insert query for each exchange using the `generate_exchange_insert` method and executes it using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `Metadata` object.
    /// * `exchanges` - A reference to a vector of `Exchange` objects containing the metadata to be imported.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - A result indicating the success of the import operation. Returns `Ok(())` if the import is successful, or an `Err` containing the error if it fails.
    ///
    pub async fn import_exchanges_metadata(
        &self,
        exchanges: &[MetaExchange],
    ) -> Result<(), ClickHouseUtilError> {
        for exchange in exchanges {
            let insert_query = self.generate_exchange_insert(exchange);
            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset");
        }

        Ok(())
    }
}

impl Metadata {
    /// Generates a `ClickHouse` SQL query to insert an exchange into the exchanges table.
    ///
    /// This method generates a SQL query that can be used to insert an exchange into the exchanges table.
    ///
    /// # Arguments
    ///
    /// * `exchange` - A reference to an `Exchange` object that contains the exchange data to be inserted.
    ///
    /// # Returns
    ///
    /// * `String` - The generated SQL query.
    ///
    pub(crate) fn generate_exchange_insert(&self, exchange: &MetaExchange) -> String {
        let table_name = format!("{DB_NAME}.{EXCHANGES_TABLE}");
        let code = &exchange.code;
        let name = &exchange.name;
        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
         )
         "
        )
    }
}
