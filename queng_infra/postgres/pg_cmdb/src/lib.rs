use diesel::r2d2::R2D2Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub mod model;
pub(crate) mod schema;

pub type Connection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_cmdb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check DB connection!
    match conn.ping() {
        Ok(_) => {}
        Err(e) => {
            eprint!("[pg_cmdb]: Error connecting to database: {}", e);
            return Err(Box::new(e));
        }
    }

    // Check if DB has pending migrations
    let has_pending = match conn.has_pending_migration(MIGRATIONS) {
        Ok(has_pending) => has_pending,
        Err(e) => {
            eprint!(
                "[pg_cmdb]: Error checking for pending database migrations: {}",
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
                eprint!("[pg_cmdb]: Error migrating database: {}", e);
                Err(e)
            }
        }
    } else {
        // If nothing pending, just return
        Ok(())
    }
}
