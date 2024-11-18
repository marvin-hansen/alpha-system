use common_errors::ValidationError;
use std::error::Error;
use std::fmt;

/// Custom error type for DB query errors
#[derive(Debug)]
pub enum ClickHouseQueryError {
    QueryFailed(String),
    InvalidTableName(ValidationError),
    EmptyTableName(ValidationError),
    TableNameTooLong(ValidationError),
    TableDoesNotExist(String, String),
}

impl Error for ClickHouseQueryError {}

impl fmt::Display for ClickHouseQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClickHouseQueryError::QueryFailed(e) =>
                write!(f, "Query to DB failed: {e}"),

            ClickHouseQueryError::InvalidTableName(e) =>
                write!(f, "Invalid table name provided: Only use alphanumeric characters and underscores as table name. Error: {e}"),

            ClickHouseQueryError::EmptyTableName(e) =>
                write!(f, "Empty table name provided: Table must have a name. Error: {e}"),

            ClickHouseQueryError::TableNameTooLong(e) =>
                write!(f, "Table name exceeds maximum length: Table can only be 63 characters long. Error: {e}"),

            ClickHouseQueryError::TableDoesNotExist(table_name, e) =>
                write!(f, "Table does not exist: Table {table_name} does not exist. Error: {e}"),
        }
    }
}
