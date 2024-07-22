use crate::db::metadata::{Metadata, DB_NAME};
use crate::error::ClickHouseUtilError;

impl Metadata {
    pub(crate) async fn drop_metadata_db(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = format!("DROP DATABASE IF EXISTS {DB_NAME}");

        match self.execute_query(&ddl).await {
            Ok(_) => (),
            Err(e) => return Err(ClickHouseUtilError::from(e.to_string())),
        };

        Ok(())
    }
}
