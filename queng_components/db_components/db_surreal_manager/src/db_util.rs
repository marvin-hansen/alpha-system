use crate::error::SurrealDBError;
use crate::SurrealDBManager;

impl SurrealDBManager {
    pub async fn query(&self, query: &str) -> Result<(), SurrealDBError> {
        match self.db.query(query).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealDBError::QueryFailed(e.to_string())),
        }
    }
}
