use crate::db::metadata::{Metadata, DB_NAME};
use std::error::Error;

impl Metadata {
    pub(crate) async fn drop_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = format!("DROP DATABASE IF EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }
}
