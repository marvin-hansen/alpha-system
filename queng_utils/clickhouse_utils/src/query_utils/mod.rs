use crate::types::error::QueryError;
use crate::types::{CountRow, ExistsDBRow, ExistsRow};
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
///
/// This function takes a reference to a `Client` object and a query string as input. It uses the `execute` method of the `Client` to execute the query.
///
/// # Arguments
///
/// * `client` - A reference to a `Client` object.
/// * `query` - A string containing the query to be executed.
///
/// # Returns
///
/// * `Result<(), QueryError>` - Returns `Ok(())` if the query is executed successfully, or an `Err` containing a `QueryError` if it fails.
///
/// # Errors
///
/// Returns a `QueryError` if there is an error executing the query.
///
pub(crate) async fn execute_query(client: &Client, query: &str) -> Result<(), QueryError> {
    //
    let res = client.execute(query).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(QueryError::QueryFailed(e.to_string())),
    }
}

/// Verifies the existence of a table in the ClickHouse database.
///
/// This function takes a reference to a `Client` object and a query string as input. It queries the database to check if the table exists.
///
/// # Arguments
///
/// * `client` - A reference to a `Client` object.
/// * `query` - A string containing the query to check the existence of the table.
///
/// # Returns
///
/// * `Result<bool, QueryError>` - Returns `Ok(true)` if the table exists, `Ok(false)` if it does not exist, or an `Err` containing a `QueryError` if the query fails.
///
/// # Errors
///
/// Returns a `QueryError` if there is an error executing the query.
///
pub(crate) async fn verify_table_exists(client: &Client, query: &str) -> Result<bool, QueryError> {
    //
    let res: Result<ExistsRow, KlickhouseError> = client.query_one(query).await;

    match res {
        Ok(res) => Ok(res.exists()),
        Err(e) => Err(QueryError::QueryFailed(e.to_string())),
    }
}

pub(crate) async fn verify_db_exists(client: &Client, db_name: &str) -> Result<bool, QueryError> {
    let query = format!("show databases like '{db_name}'");

    let res: Result<ExistsDBRow, KlickhouseError> = client.query_one(query).await;
    return match res {
        Ok(row) => {
            let val = row.value();

            let db_exists = !val.is_empty() && val.eq_ignore_ascii_case(db_name);

            Ok(db_exists)
        }
        Err(_) => Ok(false),
    };
}

/// Counts the number of rows in a specified table in the ClickHouse database.
///
/// This function takes a reference to a `Client` object and the name of the table as input. It generates a `SELECT count(*)` query for the table and executes it to get the number of rows.
///
/// # Arguments
///
/// * `client` - A reference to a `Client` object.
/// * `table_name` - A string containing the name of the table to count rows from.
///
/// # Returns
///
/// * `Result<u64, QueryError>` - Returns the count of rows in the table as `u64` if successful, or an `Err` containing a `QueryError` if it fails.
///
/// # Errors
///
/// Returns a `QueryError` if there is an error executing the query.
///
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
