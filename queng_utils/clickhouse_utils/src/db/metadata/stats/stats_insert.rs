use crate::db::metadata::{Metadata, DB_NAME};
use common::prelude::Stats;

impl Metadata {
    pub fn generate_stats_insert(&self, stats: &Stats) -> String {
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
