/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::{DB_NAME, Metadata, STATS_TABLE};
use common_metadata::MetaStats;
use std::error::Error;

impl Metadata {
    /// Imports the metadata of a `Stats` object into the metadata database.
    ///
    /// This method takes a reference to a `Stats` object and imports its metadata into the metadata database.
    /// It generates an SQL insert query for the `Stats` object using the `generate_stats_insert` method.
    /// The generated query is then executed using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `stats` - A reference to the `Stats` object containing the metadata to be imported.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - A result indicating the success of the import operation. Returns `Ok(())` if the import is successful, or an `Err` containing the error if it fails.
    ///
    pub async fn import_stats_metadata(&self, stats: &MetaStats) -> Result<(), Box<dyn Error>> {
        let insert_query = self.generate_stats_insert(stats);
        self.execute_query(&insert_query)
            .await
            .expect("Failed to insert metadata stats");

        Ok(())
    }

    /// Generates an SQL insert query for inserting stats metadata into the stats table.
    ///
    /// This method takes a reference to a `Stats` object and generates an SQL insert query for inserting its metadata into the stats table.
    ///
    /// # Arguments
    ///
    /// * `stats` - A reference to the `Stats` object for which the insert query is generated.
    ///
    /// # Returns
    ///
    /// * `String` - The SQL insert query as a string.
    ///
    pub(crate) fn generate_stats_insert(&self, stats: &MetaStats) -> String {
        let table_name = format!("{DB_NAME}.{STATS_TABLE}");

        let download_timestamp = stats.download_timestamp();
        let hash = stats.hash();
        let number_assets = stats.number_assets();
        let number_exchanges = stats.number_exchanges();
        let number_instruments = stats.number_instruments();

        format!(
            r"
            INSERT INTO {table_name} (*)
            VALUES (
              '{download_timestamp}',
              '{hash}',
              {number_assets},
              {number_exchanges},
              {number_instruments},
            )
            "
        )
    }
}
