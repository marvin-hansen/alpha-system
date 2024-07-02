use crate::db::metadata::{Metadata, DB_NAME, DB_TABLES};
use crate::error::ClickHouseUtilError;

impl Metadata {
    pub(crate) fn metadata_tables(&self) -> [&'static str; 4] {
        DB_TABLES
    }
}

impl Metadata {
    pub(crate) async fn verify_metadata_tables_created(&self) -> Result<bool, ClickHouseUtilError> {
        let tables = self.metadata_tables();
        for table in tables {
            let query = self.generate_table_exists_query(table);
            match self.verify_table_exists(&query).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(ClickHouseUtilError::from(e.to_string())),
            }
        }

        Ok(true)
    }
    fn generate_table_exists_query(&self, table_name: &str) -> String {
        format!("EXISTS TABLE {DB_NAME}.{table_name};")
    }
}
