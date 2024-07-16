use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum PostgresDBError {
    ConnectionFailed(String),
    LoginFailed(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableDoesNotExist(String, String),
    TableSanitizeError(String),
    UnknownError(String),
    NotImplementedError(String),
}

impl Error for PostgresDBError {}

impl fmt::Display for PostgresDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PostgresDBError::ConnectionFailed(e) => {
                write!(f, "[SurrealDBError]: Connection to DB failed: {e}")
            }

            PostgresDBError::LoginFailed(e) => {
                write!(f, "[SurrealDBError]: Login into DB failed: {e}")
            }

            PostgresDBError::InsertFailed(e) => {
                write!(f, "[SurrealDBError]: Insert into DB failed: {e}")
            }

            PostgresDBError::UpdateFailed(e) => {
                write!(f, "[SurrealDBError]: DB Update failed: {e}")
            }

            PostgresDBError::DeleteFailed(e) => {
                write!(f, "[SurrealDBError]: Delete failed: {e}")
            }
            PostgresDBError::QueryFailed(e) => {
                write!(f, "[SurrealDBError]: DB Query failed: {e}")
            }

            PostgresDBError::TableDoesNotExist(table_name, err) => {
                write!(
                    f,
                    "Table does not exist: Table {table_name} does not exist. Error: {err}"
                )
            }

            PostgresDBError::TableSanitizeError(e) => {
                write!(f, "[SurrealDBError]: Table sanitization error: {e}")
            }

            PostgresDBError::UnknownError(e) => {
                write!(f, "[SurrealDBError]: Unknown error: {e}")
            }

            PostgresDBError::NotImplementedError(e) => {
                write!(f, "[SurrealDBError]: Not Implemented error: {e}")
            }
        }
    }
}
