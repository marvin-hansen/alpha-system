use crate::embed_migrations::EMBEDDED_MIGRATIONS;
use diesel::pg;
use diesel_migrations::EmbeddedMigrations;
use std::error::Error;

mod embed_migrations;
mod model;
pub mod prelude;

pub type Connection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<pg::PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = EMBEDDED_MIGRATIONS;

pub fn run_mddb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check if DB has pending migrations
    let has_pending = postgres_migrations::check_db_has_pending_migration(conn, MIGRATIONS)?;

    // Run migrations if there are pending
    postgres_migrations::run_db_migration(conn, MIGRATIONS, has_pending)
}

pub fn check_mddb_db_migration(
    conn: &mut Connection,
) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
    postgres_migrations::check_db_migration(conn)
}

pub fn revert_mddb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    postgres_migrations::revert_db_migration(conn, MIGRATIONS)
}
