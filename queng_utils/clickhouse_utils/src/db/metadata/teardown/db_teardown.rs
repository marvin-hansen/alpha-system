use crate::db::metadata::{Metadata, DB_NAME};
use crate::query_utils;
use std::error::Error;

impl Metadata {
    pub(crate) async fn drop_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.drop_metadata_ddl();
        query_utils::execute_query(&self.client, &ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }
    fn drop_metadata_ddl(&self) -> String {
        format!("DROP DATABASE IF EXISTS {DB_NAME}")
    }
}
