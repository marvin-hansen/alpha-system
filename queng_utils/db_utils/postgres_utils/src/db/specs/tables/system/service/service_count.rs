use crate::db::all_db_constants::SERVICE_TABLE;
use crate::db::common_queries::query;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn count_services(&self) -> Result<u64, PostgresUtilError> {
        let query = query::generate_count_table_query(SERVICE_TABLE);
        match self.execute_count_query(&query).await {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
