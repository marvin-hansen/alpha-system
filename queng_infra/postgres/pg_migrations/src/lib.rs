use diesel::pg;
use diesel::r2d2::R2D2Connection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::error::Error;

/// Runs all pending database migrations.
///
/// This function runs all pending database migrations.
///
/// # Arguments
///
/// * `conn` - A reference to a mutable `Connection` object that represents the
///   connection to the database.
/// * `embedded_migrations` - A reference to an `EmbeddedMigrations` object that
///   contains all the migrations to apply.
/// * `has_pending` - A boolean that indicates if there are pending migrations.
///
/// # Errors
///
/// Returns an `Err` variant of `Box<dyn Error + Send + Sync + 'static>` if any of the
/// migration operations fail.
///
pub fn run_db_migration(
    conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>,
    embedded_migrations: EmbeddedMigrations,
    has_pending: bool,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check DB connection!
    check_db_connection(conn)?;

    // If so, run all pending migrations.
    if has_pending {
        match conn.run_pending_migrations(embedded_migrations) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprint!("[pg_smdb]: Error migrating database: {}", e);
                Err(e)
            }
        }
    } else {
        // If nothing pending, just return
        Ok(())
    }
}

/// Checks if the database has any pending migrations.
///
/// This function checks if the database has any pending migrations and
/// returns `true` if there are, and `false` if there are not.
///
/// # Arguments
///
/// * `conn` - A reference to a mutable `Connection` object that represents the
///   connection to the database.
///
pub fn check_db_has_pending_migration(
    conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>,
    embedded_migrations: EmbeddedMigrations,
) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
    match conn.has_pending_migration(embedded_migrations) {
        Ok(has_pending) => Ok(has_pending),
        Err(e) => {
            eprint!(
                "[pg_smdb]: Error checking for pending database migrations: {}",
                e
            );
            Err(e)
        }
    }
}

/// Checks if the database has already been migrated.
///
/// This function checks if there are any pending database migrations and
/// returns `true` if there are, and `false` if there are not.
///
/// # Arguments
///
/// * `conn` - A reference to a mutable `Connection` object that represents the
///   connection to the database.
///
/// # Returns
///
/// Returns a `Result` that indicates if there are any pending database migrations.
/// If there are pending migrations, it returns `Ok(true)`.
/// If there are no pending migrations, it returns `Ok(false)`.
/// If an error occurs, it returns `Err(Box<dyn Error + Send + Sync + 'static>)`.
///
pub fn check_db_migration(
    conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>,
) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
    // Check DB connection!
    check_db_connection(conn)?;

    // Load  applied migrations from DB
    let applied_migrations = match conn.applied_migrations() {
        Ok(migrations) => migrations,
        Err(e) => {
            // Because we have already checked the DB connection,
            // this can only fail if the DB, schema, or table isn't set up properly.
            // Still print the error for debugging purposes
            eprint!(
                "[pg_smdb]: Error loading migrations from the database: {}",
                e
            );
            return Ok(false);
        }
    };

    if applied_migrations.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Revert SMDB database migrations.
///
/// This function reverts all unapplied database migrations.
///
/// # Errors
///
/// Returns an `Err` variant of `Box<dyn Error + Send + Sync + 'static>` if any of the
/// revert operations fail.
///
pub fn revert_db_migration(
    conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>,
    embedded_migrations: EmbeddedMigrations,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check DB connection!
    check_db_connection(conn)?;

    // revert all pending migrations
    match conn.revert_all_migrations(embedded_migrations) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("Error reverting database migrations: {}", e);
            Err(e)
        }
    }
}

/// Checks if a database connection is live.
///
/// This function is a simple wrapper around calling `ping` on a connection.
/// It returns `Ok(())` if the connection is live, and `Err` if it is not.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PooledConnection` object.
///
/// # Returns
///
/// Returns a `Result` that indicates if the connection is live.
/// If the connection is live, it returns `Ok(())`.
///
fn check_db_connection(
    conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    match conn.ping() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("Error connecting to database: {}", e);
            Err(Box::new(e))
        }
    }
}
