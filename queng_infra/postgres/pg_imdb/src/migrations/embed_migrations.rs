use crate::migrations::{DIESEL_DOWN, DIESEL_NAME, DIESEL_UP, SCHEMA_DOWN, SCHEMA_NAME, SCHEMA_UP};
use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, SCHEMA_MIGRATION]);

const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

const SCHEMA_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    SCHEMA_UP,
    Some(SCHEMA_DOWN),
    EmbeddedName::new(SCHEMA_NAME),
    TomlMetadataWrapper::new(true),
);
