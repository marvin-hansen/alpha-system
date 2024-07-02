use crate::db::metadata::{Metadata, DB_NAME, DB_TABLES};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    /// Counts the number of instruments in the instruments table of the metadata database.
    ///
    /// This method counts the number of instruments in the instruments table of the metadata database.
    /// It returns the count as a `u64`.
    ///
    /// # Returns
    ///
    pub async fn count_instruments(&self) -> Result<u64, ClickHouseUtilError> {
        let table_name = &format!("{}.{}", DB_NAME, DB_TABLES[3]);

        return match self.count_rows(table_name).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }
}
