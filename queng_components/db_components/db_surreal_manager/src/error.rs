use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum SurrealDBError {
    ConnectionFailed(String),
    LoginFailed(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableDoesNotExist(String, String),
    UnknownError(String),
}

impl Error for SurrealDBError {}

impl fmt::Display for SurrealDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SurrealDBError::ConnectionFailed(e) => {
                write!(f, "[SurrealDBError]: Connection to DB failed: {e}")
            }

            SurrealDBError::LoginFailed(e) => {
                write!(f, "[SurrealDBError]: Login into DB failed: {e}")
            }

            SurrealDBError::InsertFailed(e) => {
                write!(f, "[SurrealDBError]: Insert into DB failed: {e}")
            }

            SurrealDBError::UpdateFailed(e) => write!(f, "[SurrealDBError]: DB Update failed: {e}"),

            SurrealDBError::DeleteFailed(e) => write!(f, "[SurrealDBError]: Delete failed: {e}"),

            SurrealDBError::QueryFailed(e) => write!(f, "[SurrealDBError]: DB Query failed: {e}"),

            SurrealDBError::TableDoesNotExist(table_name, err) => write!(
                f,
                "Table does not exist: Table {table_name} does not exist. Error: {err}"
            ),

            SurrealDBError::UnknownError(e) => write!(f, "[SurrealDBError]: Unknown error: {e}"),
        }
    }
}
