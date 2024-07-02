use crate::db::metadata::Metadata;
use common::prelude::Stats;
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
    pub async fn import_stats_metadata(&self, stats: &Stats) -> Result<(), Box<dyn Error>> {
        let insert_query = self.generate_stats_insert(stats);
        self.execute_query(&insert_query)
            .await
            .expect("Failed to insert metadata stats");

        Ok(())
    }
}
