use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PostgresDBError {
    ConnectionFailed(String),
    CountFailed(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableDoesNotExist(String, String),
    PortfolioDoesNotExist(String),
    ServiceDoesNotExist(String),
    TableSanitizeError(String),
    UnknownError(String),
    NotImplementedError(String),
}

impl Error for PostgresDBError {}

impl fmt::Display for PostgresDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PostgresDBError::ConnectionFailed(e) => {
                write!(
                    f,
                    "[PostgresDBError]: Connection to Postgres DB failed with error: {e}"
                )
            }

            PostgresDBError::CountFailed(e) => {
                write!(f, "[PostgresDBError]: Count of DB Table has failed: {e}")
            }

            PostgresDBError::InsertFailed(e) => {
                write!(f, "[PostgresDBError]: Insert into DB failed: {e}")
            }

            PostgresDBError::UpdateFailed(e) => {
                write!(f, "[PostgresDBError]: DB Update failed: {e}")
            }

            PostgresDBError::DeleteFailed(e) => {
                write!(f, "[PostgresDBError]: Delete failed: {e}")
            }
            PostgresDBError::QueryFailed(e) => {
                write!(f, "[PostgresDBError]: DB Query failed: {e}")
            }

            PostgresDBError::TableDoesNotExist(table_name, err) => {
                write!(
                    f,
                    "Table does not exist: Table {table_name} does not exist. Error: {err}"
                )
            }

            PostgresDBError::TableSanitizeError(e) => {
                write!(f, "[PostgresDBError]: Table sanitization error: {e}")
            }

            PostgresDBError::UnknownError(e) => {
                write!(f, "[PostgresDBError]: Unknown error: {e}")
            }

            PostgresDBError::NotImplementedError(e) => {
                write!(f, "[PostgresDBError]: Not Implemented error: {e}")
            }
            PostgresDBError::PortfolioDoesNotExist(e) => {
                write!(f, "[PostgresDBError]: Portfolio does not exist: {e}")
            }
            PostgresDBError::ServiceDoesNotExist(e) => {
                write!(f, "[PostgresDBError]: Service does not exist: {e}")
            }
        }
    }
}
