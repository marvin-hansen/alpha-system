use crate::db::specs::queries::query;
use crate::db::{Specs, PORTFOLIO_TABLE};
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn count_portfolios(&self) -> Result<u64, PostgresUtilError> {
        let query = query::generate_count_table_query(PORTFOLIO_TABLE);
        match self.execute_count_query(&query).await {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
