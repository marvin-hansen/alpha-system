mod db_prtf;
mod db_svc;
mod db_util;
pub mod error;
pub mod prelude;

use std::fmt::{Debug, Display, Formatter};

use crate::error::PostgresDBError;

#[derive(Clone, Debug)]
pub struct SurrealDBManager {}

impl SurrealDBManager {
    pub async fn new() -> Result<Self, PostgresDBError> {
        Ok(Self {})
    }
}

impl SurrealDBManager {
    pub async fn is_healthy(&self) -> Result<(), PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }
}

impl Display for SurrealDBManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SurrealDBManager:",)
    }
}
