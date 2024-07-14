use crate::db::metadata::{Metadata, DB_NAME, INSTRUMENTS_TABLE};
use crate::error::ClickHouseUtilError;

impl Metadata {
    /// Counts the number of instruments in the instruments table of the metadata database.
    ///
    /// This method counts the number of instruments in the instruments table of the metadata database.
    /// It returns the count as a `u64`.
    ///
    /// # Returns
    ///
    pub async fn count_instruments(&self) -> Result<u64, ClickHouseUtilError> {
        let table_name = &format!("{}.{}", DB_NAME, INSTRUMENTS_TABLE);

        match self.count_rows(table_name).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }
}
