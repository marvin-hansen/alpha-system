use crate::db::common_ddl::ddl_db;
use crate::db::specs::{Specs, DB_NAME};
use crate::db::utils;
use crate::error::ClickHouseQueryError;

impl Specs {
    pub(crate) fn generate_drop_table_ddl(&self, table_name: &str) -> String {
        ddl_db::generate_drop_table_ddl(table_name, DB_NAME)
    }

    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), ClickHouseQueryError> {
        utils::execute_query(&self.client, query)
            .await
            .expect("Failed to query metadata DB");

        Ok(())
    }
    pub(crate) async fn verify_table_exists(
        &self,
        query: &str,
    ) -> Result<bool, ClickHouseQueryError> {
        let res = utils::verify_table_exists(&self.client, query)
            .await
            .expect("Failed to verify that table exists in metadata DB");

        Ok(res)
    }

    pub(crate) async fn verify_db_exists(
        &self,
        db_name: &str,
    ) -> Result<bool, ClickHouseQueryError> {
        let res = utils::verify_db_exists(&self.client, db_name)
            .await
            .expect("Failed to verify that table exists in metadata DB");

        Ok(res)
    }

    pub(crate) async fn count_rows(&self, table_name: &str) -> Result<u64, ClickHouseQueryError> {
        let table_name = &format!("{}.{}", DB_NAME, table_name);

        let res = utils::count_rows(&self.client, table_name)
            .await
            .expect("Failed to count table rows in metadata DB");

        Ok(res)
    }
}
