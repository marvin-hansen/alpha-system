use crate::ClickhouseUtil;
use common::prelude::Stats;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn import_stats_metadata(&self, stats: &Stats) -> Result<(), Box<dyn Error>> {
        let insert_query = self.metadata.generate_stats_insert(stats);
        self.execute_query(&insert_query)
            .await
            .expect("Failed to insert metadata stats");

        Ok(())
    }
}
