use crate::db::metadata::Metadata;
use common::prelude::Stats;
use std::error::Error;

impl Metadata {
    pub async fn import_stats_metadata(&self, stats: &Stats) -> Result<(), Box<dyn Error>> {
        let insert_query = self.generate_stats_insert(stats);
        self.execute_query(&insert_query)
            .await
            .expect("Failed to insert metadata stats");

        Ok(())
    }
}
