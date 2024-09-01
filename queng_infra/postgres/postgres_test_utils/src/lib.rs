pub mod prelude;
pub(crate) mod setup;
pub(crate) mod teardown;
pub(crate) mod utils;

use common_errors::prelude::PostgresDBError;
use diesel::{Connection, PgConnection};
use std::time::Duration;
use tokio::time::{sleep, Instant};

pub const DB_TEST_URL: &str = "postgres://postgres:postgres@localhost/postgres";

/// Connects to a Postgres database and waits for it to come online if it's not already.
///
/// # Arguments
///
/// * `database_url` - The Postgres database connection URL.
/// * `timeout` - The timeout in seconds to wait for the connection. If not provided,
/// defaults to 120 seconds (2 minutes).
///
/// # Returns
///
/// * `Result<PgConnection, PostgresDBError>` - A result indicating success or failure.
/// If successful, returns a `PgConnection` instance.
/// If the connection fails, returns a `PostgresDBError` indicating the failure.
///
pub async fn get_or_wait_for_postgres_connection(
    database_url: &str,
    timeout: Option<u64>,
) -> Result<PgConnection, PostgresDBError> {
    get_or_wait_for_postgres_database_connection(database_url, timeout, false).await
}

/// Connects to a Postgres database and waits for it to come online if it's not already.
/// Unlike the regular connection check, this one retries at a much shorter interval
/// to ensure a DB migration completes before any regular connection can be established.
/// Use this for Database migration or any test setup.
///
/// # Arguments
///
/// * `database_url` - The Postgres database connection URL.
/// * `timeout` - The timeout in seconds to wait for the connection. If not provided,
/// defaults to 120 seconds (2 minutes).
///
/// # Returns
///
/// * `Result<PgConnection, PostgresDBError>` - A result indicating success or failure.
/// If successful, returns a `PgConnection` instance.
/// If the connection fails, returns a `PostgresDBError` indicating the failure.
///
pub async fn get_or_wait_for_postgres_migration_connection(
    database_url: &str,
    timeout: Option<u64>,
) -> Result<PgConnection, PostgresDBError> {
    get_or_wait_for_postgres_database_connection(database_url, timeout, true).await
}

async fn get_or_wait_for_postgres_database_connection(
    database_url: &str,
    timeout: Option<u64>,
    migration: bool,
) -> Result<PgConnection, PostgresDBError> {
    let start_time = Instant::now();
    let retry_interval = if migration {
        Duration::from_millis(50)
    } else {
        Duration::from_millis(500)
    };

    let timeout = Duration::from_secs(timeout.unwrap_or(120));

    loop {
        match PgConnection::establish(&database_url) {
            Ok(conn) => return Ok(conn),
            Err(_) => {
                if start_time.elapsed() > timeout {
                    return Err(PostgresDBError::ConnectionFailed(format!(
                        "Failed to connect to Postgres server at: {} after several retries for 90 seconds",
                        database_url
                    )));
                }
                sleep(retry_interval).await;
            }
        }
    }
}
