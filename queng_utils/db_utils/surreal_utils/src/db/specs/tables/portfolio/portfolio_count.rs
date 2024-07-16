use crate::db::Specs;
use crate::prelude::SurrealUtilError;

impl Specs {
    pub async fn count_portfolios(&self) -> Result<u64, SurrealUtilError> {
        return Err(SurrealUtilError::from("Not implemented".to_string()));
    }
}
