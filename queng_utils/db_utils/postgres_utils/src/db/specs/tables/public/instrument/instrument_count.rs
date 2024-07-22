use crate::common::all_db_constants::INSTRUMENT_TABLE;
use crate::common::common_queries::query;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn count_instruments(&self) -> Result<u64, PostgresUtilError> {
        self.dbg_print("count_instruments");
        let query = query::generate_count_table_query(INSTRUMENT_TABLE);
        match self.execute_count_query(&query).await {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
