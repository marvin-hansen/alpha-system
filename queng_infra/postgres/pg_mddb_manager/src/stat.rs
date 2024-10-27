use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use common_metadata::prelude::MetaDataDBRecords;

impl PostgresMDDBManager {
    ///
    /// Counts the metadata records in the database.
    ///
    /// This asynchronous function counts the number of assets, exchanges, and
    /// instruments present in the database and returns a `MetaDataDBRecords`
    /// struct containing these counts. If any error occurs during the counting
    /// process, a `PostgresDBError` is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(MetaDataDBRecords)` - A struct with the counts of assets,
    ///   exchanges, and instruments.
    /// * `Err(PostgresDBError)` - If an error occurs while counting the records.
    ///
    pub async fn count_metadata_records(&self) -> Result<MetaDataDBRecords, PostgresDBError> {
        self.dbg_print("insert_instruments");

        // We cast into u32 to ensure compability with the MetaDataSet struct from the Kaiko proxy
        let db_asset_count = self.count_assets().await.expect("Failed to count assets") as u32;

        let db_exchange_count = self
            .count_exchanges()
            .await
            .expect("Failed to count exchanges") as u32;

        let db_instrument_count = self
            .count_instruments()
            .await
            .expect("Failed to count instruments") as u32;

        Ok(MetaDataDBRecords::new(
            db_asset_count,
            db_exchange_count,
            db_instrument_count,
        ))
    }
}
