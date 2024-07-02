use crate::db::metadata::{Metadata, DB_NAME, DB_TABLES};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    pub async fn count_exchanges(&self) -> Result<u64, ClickHouseUtilError> {
        let table_name = &format!("{}.{}", DB_NAME, DB_TABLES[1]);

        return match self.count_rows(table_name).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }
}
