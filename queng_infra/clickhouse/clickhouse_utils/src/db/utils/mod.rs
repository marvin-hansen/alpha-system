/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::error::ClickHouseQueryError;
use crate::types::{CountRow, ExistsDBRow, ExistsRow};
use klickhouse::{Client, KlickhouseError};

/// Executes a query on the specified table in the `ClickHouse` database.
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
pub(crate) async fn execute_query(
    client: &Client,
    query: &str,
) -> Result<(), ClickHouseQueryError> {
    //
    let res = client.execute(query).await;
    match res {
        Ok(()) => Ok(()),
        Err(e) => Err(ClickHouseQueryError::QueryFailed(e.to_string())),
    }
}

/// Verifies the existence of a table in the `ClickHouse` database.
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
pub(crate) async fn verify_table_exists(
    client: &Client,
    query: &str,
) -> Result<bool, ClickHouseQueryError> {
    //
    let res: Result<ExistsRow, KlickhouseError> = client.query_one(query).await;

    match res {
        Ok(res) => Ok(res.exists()),
        Err(e) => Err(ClickHouseQueryError::QueryFailed(e.to_string())),
    }
}

/// Verifies the existence of a database in `ClickHouse`.
///
/// This method takes a reference to a `Client` object and the name of the database
/// to verify. It generates a query to check if the database exists in `ClickHouse` and
/// executes the query using the `client` provided. If the database exists, it returns
/// [Ok(true)](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_utils/db_utils/postgres_utils/src/db/specs/db_setup/setup_db.rs:61:0-77:0), otherwise it returns [Ok(false)](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_utils/db_utils/postgres_utils/src/db/specs/db_setup/setup_db.rs:61:0-77:0). If there is an error executing the query,
/// it returns an [Err](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_utils/db_utils/clickhouse_utils/src/db/all_db_verify.rs:0:0-23:0) containing a `ClickHouseQueryError`.
///
/// # Arguments
///
/// * `client` - A reference to a `Client` object connected to `ClickHouse`.
/// * `db_name` - A string containing the name of the database to verify.
///
/// # Returns
///
/// * [Ok(true)](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_utils/db_utils/postgres_utils/src/db/specs/db_setup/setup_db.rs:61:0-77:0) if the database exists.
/// * [Ok(false)](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_utils/db_utils/postgres_utils/src/db/specs/db_setup/setup_db.rs:61:0-77:0) if the database does not exist.
/// * `Err(ClickHouseQueryError)` if there is an error executing the query.
///
pub(crate) async fn verify_db_exists(
    client: &Client,
    db_name: &str,
) -> Result<bool, ClickHouseQueryError> {
    let query = format!("show databases like '{db_name}'");

    let res: Result<ExistsDBRow, KlickhouseError> = client.query_one(query).await;
    match res {
        Ok(row) => {
            let val = row.value();

            let db_exists = !val.is_empty() && val.eq_ignore_ascii_case(db_name);

            Ok(db_exists)
        }
        Err(_) => Ok(false),
    }
}

/// Counts the number of rows in a specified table in the `ClickHouse` database.
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
pub(crate) async fn count_rows(
    client: &Client,
    table_name: &str,
) -> Result<u64, ClickHouseQueryError> {
    //
    let count_query = format!("SELECT count(*) FROM {table_name}");

    // We need type annotation of the Result type here.
    let number_of_rows: Result<CountRow, KlickhouseError> = client.query_one(&count_query).await;

    match number_of_rows {
        Ok(number_of_rows) => Ok(number_of_rows.count()),
        Err(e) => Err(ClickHouseQueryError::QueryFailed(e.to_string())),
    }
}
