use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use diesel::{Connection, PgConnection};
use std::error::Error;
use std::time::Duration;

/// A connection customizer designed for use in tests.
/// Implements CustomizeConnection in a way
/// that ensures transactions in a pool customized by it are never committed.
///
/// During testing, set pool size to 1 to ensure that the same test transaction
/// is re-used throughout the test.
///
/// See:
/// <https://github.com/diesel-rs/diesel/discussions/4323#discussioncomment-11087060>
/// <https://docs.diesel.rs/master/diesel/r2d2/struct.TestCustomizer.html>
#[derive(Debug)]
pub struct TestConnectionCustomizer;

impl<E> CustomizeConnection<PgConnection, E> for TestConnectionCustomizer {
    /// Starts a test transaction whenever a connection is acquired.
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), E> {
        conn.begin_test_transaction()
            .expect("Failed to start test transaction");

        Ok(())
    }
}

/// Builds a Postgres Connection Pool based on the provided configuration parameters.
///
/// This function handles the following workflow:
/// 1. Establishes a connection pool to the Postgres database using the provided URL.
/// 2. Optionally enables debug mode to print debug messages.
/// 3. Performs database migration if the `test` parameter is set to true.
///
/// # Arguments
///
/// * `test` - A boolean indicating whether to start a test transaction.
/// * `dbg` - A boolean indicating whether to enable debug mode.
/// * `max_size` - The maximum size of the connection pool.
/// * `url` - A string slice representing the URL of the Postgres database.
///
/// # Returns
///
/// A Result containing the newly created connection pool,
/// or a PostgresDBError if an error occurs.
///
pub fn build_pg_connection_pool(
    test: bool,
    dbg: bool,
    max_size: u32,
    url: &str,
) -> Result<Pool<ConnectionManager<PgConnection>>, PostgresDBError> {
    let pool = if test {
        if dbg {
            println!("Build test connection pool",);
        }
        Pool::builder()
            .test_on_check_out(true)
            .max_size(1)
            .connection_customizer(Box::new(TestConnectionCustomizer))
            .build(ConnectionManager::<PgConnection>::new(url))
            .expect("Failed to create PG pool with test transaction")
    } else {
        if dbg {
            println!("Build connection pool",);
        }

        Pool::builder()
            .test_on_check_out(true)
            .max_size(max_size)
            .idle_timeout(Some(Duration::from_secs(10 * 60)))
            .connection_timeout(Duration::from_secs(30))
            .build(ConnectionManager::<PgConnection>::new(url))
            .expect("Failed to create PG connection pool")
    };

    Ok(pool)
}

/// Builds a Postgres Connection Pool based on the provided configuration parameters.
///
/// This function handles the following workflow:
/// 1. Establishes a connection pool to the Postgres database using the provided URL.
/// 2. Optionally enables debug mode to print debug messages.
/// 3. Performs database migration if the test parameter is set to true.
///
/// # Arguments
///
/// * `dbg` - A boolean indicating whether debug mode is enabled.
/// * `test` - A boolean indicating whether testing mode is enabled.
/// * `url` - A reference to a string representing the URL of the Postgres database.
///
/// # Returns
///
/// A Result containing the constructed Postgres Connection Pool
/// or a PostgresDBError if an error occurs during the process.
///
pub fn build_pg_connection_pool_with_migration(
    test: bool,
    dbg: bool,
    url: &str,
    migration_fun: fn(
        conn: &mut PgConnection,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>,
) -> Result<Pool<ConnectionManager<PgConnection>>, PostgresDBError> {
    let pool = if test {
        if dbg {
            println!("Build test connection pool",);
        }
        Pool::builder()
            .test_on_check_out(true)
            .max_size(1)
            .connection_customizer(Box::new(TestConnectionCustomizer))
            .build(ConnectionManager::<PgConnection>::new(url))
            .expect("Failed to create PG pool with test transaction")
    } else {
        if dbg {
            println!("Build connection pool",);
        }

        Pool::builder()
            .test_on_check_out(true)
            .max_size(5)
            .idle_timeout(Some(Duration::from_secs(10 * 60)))
            .connection_timeout(Duration::from_secs(30))
            .build(ConnectionManager::<PgConnection>::new(url))
            .expect("Failed to create PG connection pool")
    };

    if dbg {
        println!("Run DB Migration",);
    }
    match migration_fun(&mut pool.get().expect("Failed to get connection from pool")) {
        Ok(_) => {}
        Err(e) => {
            return Err(PostgresDBError::MigrationFailed(e.to_string()));
        }
    }

    Ok(pool)
}
