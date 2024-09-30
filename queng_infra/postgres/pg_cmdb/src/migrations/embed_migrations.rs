use crate::migrations::cmdb_setup::{SCHEMA_DOWN, SCHEMA_NAME, SCHEMA_UP};
use crate::migrations::diesel_initial_setup::{DIESEL_DOWN, DIESEL_NAME, DIESEL_UP};
use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, SCHEMA_MIGRATION]);

/// An embedded version of the database migrations.
///
/// This can be used to inspect the DB Schema without having to run it, or to
/// create a new database with the correct schema.
pub(crate) const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

/// A migration that creates the database schema.
pub(crate) const SCHEMA_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    SCHEMA_UP,
    Some(SCHEMA_DOWN),
    EmbeddedName::new(SCHEMA_NAME),
    TomlMetadataWrapper::new(true),
);
