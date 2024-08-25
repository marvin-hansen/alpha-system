use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PostgresDBError {
    ConnectionFailed(String),
    CountFailed(String),
    CheckFailed(String),
    CheckIfExistsFailed(String),
    InsertFailed(String),
    SetFieldFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableDoesNotExist(String, String),
    DataRecordDoesNotExist(String),
    TableSanitizeError(String),
    MigrationFailed(String),
    UnknownError(String),
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

            PostgresDBError::CheckFailed(e) => {
                write!(f, "[PostgresDBError]: Check if DB Table has failed: {e}")
            }

            PostgresDBError::CheckIfExistsFailed(e) => {
                write!(f, "[PostgresDBError]: Check if DB Table exists failed: {e}")
            }

            PostgresDBError::InsertFailed(e) => {
                write!(f, "[PostgresDBError]: Insert into DB failed: {e}")
            }

            PostgresDBError::SetFieldFailed(e) => {
                write!(f, "[PostgresDBError]: Set field failed: {e}")
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

            PostgresDBError::DataRecordDoesNotExist(e) => {
                write!(f, "[PostgresDBError]: Data record does not exist: {e}")
            }

            PostgresDBError::MigrationFailed(e) => {
                write!(f, "[PostgresDBError]: Migration failed: {e}")
            }

            PostgresDBError::UnknownError(e) => {
                write!(f, "[PostgresDBError]: Unknown error: {e}")
            }
        }
    }
}
