use crate::db::metadata::{Metadata, DB_NAME};
use crate::query_utils;
use crate::query_utils::ddl_utils;
use crate::types::error::QueryError;

impl Metadata {
    pub(crate) fn generate_drop_table_ddl(&self, table_name: &str) -> String {
        ddl_utils::generate_drop_table_ddl(table_name, DB_NAME)
    }

    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), QueryError> {
        query_utils::execute_query(&self.client, &query)
            .await
            .expect("Failed to query metadata DB");

        Ok(())
    }
    pub(crate) async fn verify_table_exists(&self, query: &str) -> Result<bool, QueryError> {
        let res = query_utils::verify_table_exists(&self.client, &query)
            .await
            .expect("Failed to verify that table exists in metadata DB");

        Ok(res)
    }

    pub(crate) async fn verify_db_exists(&self, db_name: &str) -> Result<bool, QueryError> {
        let res = query_utils::verify_db_exists(&self.client, &db_name)
            .await
            .expect("Failed to verify that table exists in metadata DB");

        Ok(res)
    }

    pub(crate) async fn count_rows(&self, table_name: &str) -> Result<u64, QueryError> {
        let res = query_utils::count_rows(&self.client, table_name)
            .await
            .expect("Failed to count table rows in metadata DB");

        Ok(res)
    }
}
