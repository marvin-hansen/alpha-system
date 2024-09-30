use crate::migrations::EMBEDDED_MIGRATIONS;
// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
use diesel_migrations::EmbeddedMigrations;
use postgres_migrations::Connection;
use std::error::Error;

pub(crate) mod migrations;
pub mod model;
pub mod prelude;
pub(crate) mod schema;

pub const MIGRATIONS: EmbeddedMigrations = EMBEDDED_MIGRATIONS;
/// Run all pending migrations.
///
/// This function runs all pending database migrations.
///
/// # Arguments
///
/// * `conn`: A reference to a `Connection` object that represents the
///   connection to the database.
///
/// # Errors
///
/// Returns an `Err` variant of `Box<dyn Error + Send + Sync + 'static>` if
/// any of the migration operations fail.
///
pub fn run_cmdb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Run migrations if there are pending
    postgres_migrations::run_db_migration(conn, MIGRATIONS)
}

/// Checks if the database has already been migrated.
///
/// This function checks if there are any pending database migrations and
/// returns `true` if there are, and `false` if there are not.
///
/// # Arguments
///
/// * `conn`: A reference to a mutable `Connection` object that represents the
///   connection to the database.
///
/// # Errors
///
/// Returns an `Err` variant of `Box<dyn Error + Send + Sync + 'static>` if
/// any of the migration operations fail.
///
pub fn check_cmdb_db_migration(
    conn: &mut Connection,
) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
    postgres_migrations::check_db_migration(conn)
}

/// Revert CMDB database migrations.
///
/// This function reverts all unapplied database migrations.
///
/// # Errors
///
/// Returns an `Err` variant of `Box<dyn Error + Send + Sync + 'static>` if any of the
/// revert operations fail.
///
pub fn revert_cmdb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    postgres_migrations::revert_db_migration(conn, MIGRATIONS)
}
