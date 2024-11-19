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
            Self::ConnectionFailed(e) => {
                write!(f, "[ClickHouseDBError]: Connection to DB failed: {e}")
            }
            Self::InsertFailed(e) => {
                write!(f, "[ClickHouseDBError]: Insert into DB failed: {e}")
            }
            Self::UpdateFailed(e) => {
                write!(f, "[ClickHouseDBError]: DB Update failed: {e}")
            }
            Self::DeleteFailed(e) => {
                write!(f, "[ClickHouseDBError]: Delete failed: {e}")
            }
            Self::QueryFailed(e) => {
                write!(f, "[ClickHouseDBError]: DB Query failed: {e}")
            }
            Self::TableSanitizeError(e) => {
                write!(f, "[ClickHouseDBError]: Table sanitization error: {e}")
            }
            Self::UnknownError(e) => {
                write!(f, "[SurrealDBError]: Unknown error: {e}")
            }
        }
    }
}
