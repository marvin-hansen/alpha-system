use crate::db::metadata::{Metadata, DB_NAME};
use common::prelude::Stats;

impl Metadata {
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
    pub(crate) fn generate_stats_insert(&self, stats: &Stats) -> String {
        let table_name = format!("{DB_NAME}.stats");

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
