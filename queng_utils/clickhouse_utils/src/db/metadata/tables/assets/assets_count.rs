use crate::db::metadata::{Metadata, DB_NAME, DB_TABLES};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    /// Counts the number of assets in the assets table of the metadata database.
    ///
    /// This method counts the number of assets in the assets table of the metadata database.
    /// It returns the count as a `u64`.
    ///
    /// # Returns
    ///
    /// * `Result<u64, ClickHouseUtilError>` - The number of assets in the table, or an error if the count fails.
    ///
    pub async fn count_assets(&self) -> Result<u64, ClickHouseUtilError> {
        let table_name = &format!("{}.{}", DB_NAME, DB_TABLES[0]);

        return match self.count_rows(table_name).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }
}
