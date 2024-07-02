use crate::types::error::QueryError;
use crate::types::{CountRow, ExistsRow};
use common::prelude::ValidationError;
use klickhouse::{Client, KlickhouseError};

/// Sanitizes the provided table name to prevent SQL injection attacks.
///
/// # Arguments
///
/// * `table_name` - The table name to sanitize
///
/// # Returns
///
/// A `Result` containing the original table name if valid, or a `QueryError`
/// if the name is invalid.
///
/// # Errors
///
/// - `QueryError::EmptyTableName` if `table_name` is empty
/// - `QueryError::InvalidTableName` if `table_name` contains invalid characters
/// - `QueryError::TableNameTooLong` if `table_name` is longer than 64 characters
///
///
/// This checks `table_name` for:
///
/// - Emptiness
/// - Invalid characters
/// - Length less than 64 characters
///
/// If valid, it returns the original `table_name`.
pub fn sanitize_table_name(table_name: &str) -> Result<&str, QueryError> {
    // check for empty name
    if table_name.is_empty() {
        return Err(QueryError::EmptyTableName(ValidationError::new(format!(
            "Table: {}",
            table_name
        ))));
    }

    // check for invalid characters
    if table_name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Err(QueryError::InvalidTableName(ValidationError::new(format!(
            "Table: {}",
            table_name
        ))));
    }

    // check for length
    if table_name.len() > 64 {
        return Err(QueryError::TableNameTooLong(ValidationError::new(format!(
            "Table: {}",
            table_name
        ))));
    }

    Ok(table_name)
}

/// Executes a query on the specified table in the ClickHouse database.
pub(crate) async fn execute_query(client: &Client, query: &str) -> Result<(), QueryError> {
    //
    let res = client.execute(query).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(QueryError::QueryFailed(e.to_string())),
    }
}

/// Queries whether the table exists. Returns true if so, otherwise false.
pub(crate) async fn verify_table_exists(client: &Client, query: &str) -> Result<bool, QueryError> {
    //
    let res: Result<ExistsRow, KlickhouseError> = client.query_one(query).await;

    match res {
        Ok(res) => Ok(res.exists()),
        Err(e) => Err(QueryError::QueryFailed(e.to_string())),
    }
}

/// Counts the number of rows in the specified table in the ClickHouse database.
pub(crate) async fn count_rows(client: &Client, table_name: &str) -> Result<u64, QueryError> {
    //
    let count_query = format!("SELECT count(*) FROM {table_name}");

    // We need type annotation of the Result type here.
    let number_of_rows: Result<CountRow, KlickhouseError> = client.query_one(&count_query).await;

    match number_of_rows {
        Ok(number_of_rows) => Ok(number_of_rows.count()),
        Err(e) => Err(QueryError::QueryFailed(e.to_string())),
    }
}
