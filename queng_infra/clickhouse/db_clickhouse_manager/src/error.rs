use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ClickHouseDBError {
    ConnectionFailed(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableSanitizeError(String),
    UnknownError(String),
}

impl Error for ClickHouseDBError {}

impl fmt::Display for ClickHouseDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClickHouseDBError::ConnectionFailed(e) => {
                write!(f, "[ClickHouseDBError]: Connection to DB failed: {e}")
            }
            ClickHouseDBError::InsertFailed(e) => {
                write!(f, "[ClickHouseDBError]: Insert into DB failed: {e}")
            }
            ClickHouseDBError::UpdateFailed(e) => {
                write!(f, "[ClickHouseDBError]: DB Update failed: {e}")
            }
            ClickHouseDBError::DeleteFailed(e) => {
                write!(f, "[ClickHouseDBError]: Delete failed: {e}")
            }
            ClickHouseDBError::QueryFailed(e) => {
                write!(f, "[ClickHouseDBError]: DB Query failed: {e}")
            }
            ClickHouseDBError::TableSanitizeError(e) => {
                write!(f, "[ClickHouseDBError]: Table sanitization error: {e}")
            }
            ClickHouseDBError::UnknownError(e) => {
                write!(f, "[SurrealDBError]: Unknown error: {e}")
            }
        }
    }
}
