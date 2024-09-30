mod diesel_initial_setup;
mod smdb_setup;

use crate::migrations::diesel_initial_setup::down::DIESEL_DOWN;
use crate::migrations::diesel_initial_setup::up::DIESEL_UP;
use crate::migrations::smdb_setup::down::SCHEMA_DOWN;
use crate::migrations::smdb_setup::up::SCHEMA_UP;
use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};

pub(crate) const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, SCHEMA_MIGRATION]);

/// An embedded version of the DIESEL database migrations metadata tables.
/// This can be used to inspect the DB Schema without having to run it, or to
/// create a new database with Diesel auto-migration support.
const DIESEL_NAME: &str = "00000000000000_diesel_initial_setup";
const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

/// A migration that creates the database schema.
const SCHEMA_NAME: &str = "2024-08-12-093223_smdb";
const SCHEMA_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    SCHEMA_UP,
    Some(SCHEMA_DOWN),
    EmbeddedName::new(SCHEMA_NAME),
    TomlMetadataWrapper::new(true),
);
