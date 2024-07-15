use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SpecDBError {
    ConnectionFailed(String),
    LoginFailed(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableDoesNotExist(String, String),
    UnknownError(String),
}

impl Error for SpecDBError {}

impl Display for SpecDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SpecDBError::ConnectionFailed(e) => {
                write!(f, "[SpecDBError]: Connection to DB failed: {e}")
            }

            SpecDBError::LoginFailed(e) => {
                write!(f, "[SpecDBError]: Login into DB failed: {e}")
            }

            SpecDBError::InsertFailed(e) => {
                write!(f, "[SpecDBError]: Insert into DB failed: {e}")
            }

            SpecDBError::UpdateFailed(e) => write!(f, "[SpecDBError]: DB Update failed: {e}"),

            SpecDBError::DeleteFailed(e) => write!(f, "[SpecDBError]: Delete failed: {e}"),

            SpecDBError::QueryFailed(e) => write!(f, "[SpecDBError]: DB Query failed: {e}"),

            SpecDBError::TableDoesNotExist(table_name, err) => write!(
                f,
                "[SpecDBError]: Table does not exist: {table_name}. Error: {err}"
            ),

            SpecDBError::UnknownError(e) => write!(f, "[SpecDBError]: Unknown error: {e}"),
        }
    }
}
