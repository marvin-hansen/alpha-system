use crate::db::metadata::{Metadata, DB_NAME};
use crate::query_utils;
use std::error::Error;

impl Metadata {
    pub(crate) async fn create_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.create_metadata_ddl();

        query_utils::execute_query(&self.client, &ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }

    fn create_metadata_ddl(&self) -> String {
        format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}")
    }
}
