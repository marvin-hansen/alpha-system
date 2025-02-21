/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::{DB_NAME, EXCHANGES_TABLE, Metadata};
use crate::error::ClickHouseUtilError;

impl Metadata {
    /// Counts the number of exchanges in the exchanges table of the metadata database.
    ///
    /// This method counts the number of exchanges in the exchanges table of the metadata database.
    /// It returns the count as a `u64`.
    ///
    /// # Returns
    ///
    /// * `Result<u64, ClickHouseUtilError>` - The number of exchanges in the table, or an error if the count fails.
    ///
    pub async fn count_exchanges(&self) -> Result<u64, ClickHouseUtilError> {
        let table_name = &format!("{DB_NAME}.{EXCHANGES_TABLE}");

        match self.count_rows(table_name).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }
}
