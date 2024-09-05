use crate::embed_migrations::EMBEDDED_MIGRATIONS;
use diesel_migrations::EmbeddedMigrations;
use postgres_migrations::Connection;
use std::error::Error;

mod embed_migrations;

mod schema;

pub const MIGRATIONS: EmbeddedMigrations = EMBEDDED_MIGRATIONS;

pub fn run_mddb_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("[PostgresMddb]: Run DB Migration()");

    // Run migrations if there are pending
    postgres_migrations::run_db_migration(conn, MIGRATIONS)
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
