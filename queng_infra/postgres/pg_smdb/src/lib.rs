use crate::embed_migrations::EMBEDDED_MIGRATIONS;
use diesel::pg;
use diesel_migrations::EmbeddedMigrations;
use std::error::Error;

mod embed_migrations;
pub mod model;
pub mod prelude;
pub(crate) mod schema;

pub type Connection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>;

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
pub fn run_smdb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check if DB has pending migrations
    let has_pending = pg_migrations::check_db_has_pending_migration(conn, MIGRATIONS)?;

    // Run migrations if there are pending
    pg_migrations::run_db_migration(conn, MIGRATIONS, has_pending)
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
pub fn check_smdb_db_migration(
    conn: &mut Connection,
) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
    pg_migrations::check_db_migration(conn)
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
pub fn revert_smdb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    pg_migrations::revert_db_migration(conn, MIGRATIONS)
}
