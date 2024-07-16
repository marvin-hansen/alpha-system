use crate::error::PostgresDBError;
use crate::PostgresDBManager;

impl PostgresDBManager {
    pub async fn query(&self, _query: &str) -> Result<(), PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function query".to_string(),
        ))
    }
}
