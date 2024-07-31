use common_pg_queries::prelude::pg_query_service;

use crate::common::all_db_constants::{SERVICE_TABLE, SYSTEM_SCHEMA};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn count_services(&self) -> Result<u64, PostgresUtilError> {
        let query = pg_query_service::generate_count_table_query(SYSTEM_SCHEMA, SERVICE_TABLE);
        match self.execute_count_query(&query).await {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
