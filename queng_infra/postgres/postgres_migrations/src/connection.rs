use common_errors::PostgresDBError;
use diesel::{Connection, PgConnection};
use std::time::Duration;
use tokio::time::{sleep, Instant};

pub const DB_TEST_URL: &str = "postgres://postgres:postgres@localhost/postgres";

/// Connects to a Postgres database and waits for it to come online if it's not already.
///
/// # Arguments
///
/// * `database_url` - The Postgres database connection URL.
/// * `timeout` - The timeout in seconds to wait for the connection. If not provided, defaults to 120 seconds (2 minutes).
///
/// # Returns
///
/// `Result<PgConnection, PostgresDBError>` - A result indicating success or failure.
/// * If successful, returns a `PgConnection` instance.
/// * If the connection fails, returns a `PostgresDBError` indicating the failure.
///
pub async fn get_or_wait_for_postgres_connection(
    database_url: &str,
    timeout: Option<u64>,
) -> Result<PgConnection, PostgresDBError> {
    let start_time = Instant::now();
    let retry_interval = Duration::from_secs(1);
    let timeout = Duration::from_secs(timeout.unwrap_or(120));

    loop {
        match PgConnection::establish(database_url) {
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
