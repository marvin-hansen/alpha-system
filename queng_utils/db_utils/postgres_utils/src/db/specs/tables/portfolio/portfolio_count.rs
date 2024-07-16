use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn count_portfolios(&self) -> Result<u64, PostgresUtilError> {
        return Err(PostgresUtilError::from("Not implemented".to_string()));
    }
}
