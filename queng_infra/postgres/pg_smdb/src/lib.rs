use crate::embed_migrations::EMBEDDED_MIGRATIONS;
use diesel::pg;
use diesel::r2d2::R2D2Connection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::error::Error;

mod embed_migrations;
pub mod model;
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
    // Check DB connection!
    match conn.ping() {
        Ok(_) => {}
        Err(e) => {
            eprint!("[pg_smdb]: Error connecting to database: {}", e);
            return Err(Box::new(e));
        }
    }

    // Check if DB has pending migrations
    let has_pending = match conn.has_pending_migration(MIGRATIONS) {
        Ok(has_pending) => has_pending,
        Err(e) => {
            eprint!(
                "[pg_smdb]: Error checking for pending database migrations: {}",
                e
            );
            return Err(e);
        }
    };

    // If so, run all pending migrations.
    if has_pending {
        match conn.run_pending_migrations(MIGRATIONS) {
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
    // Check DB connection!
    if let Ok(_) = conn.ping() {
    } else if let Err(e) = conn.ping() {
        eprint!("[pg_smdb]: Error connecting to database: {}", e);
        return Err(Box::new(e));
    }

    // revert all pending migrations
    match conn.revert_all_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("[pg_smdb]: Error reverting database migrations: {}", e);
            Err(e)
        }
    }
}
