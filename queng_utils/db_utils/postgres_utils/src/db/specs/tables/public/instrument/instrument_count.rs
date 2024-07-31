use common_pg_queries::prelude::pg_query_service;

use crate::common::all_db_constants::{DEFAULT_SCHEMA, INSTRUMENT_TABLE};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn count_instruments(&self) -> Result<u64, PostgresUtilError> {
        self.dbg_print("count_instruments");
        let query = pg_query_service::generate_count_table_query(DEFAULT_SCHEMA, INSTRUMENT_TABLE);
        match self.execute_count_query(&query).await {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
