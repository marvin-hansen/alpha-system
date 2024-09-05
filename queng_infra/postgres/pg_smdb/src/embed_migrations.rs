use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};
use std::env;

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, SCHEMA_MIGRATION]);

pub(crate) const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

pub const SCHEMA_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    SCHEMA_UP,
    Some(SCHEMA_DOWN),
    EmbeddedName::new(SCHEMA_NAME),
    TomlMetadataWrapper::new(true),
);

const DIESEL_NAME: &str = "00000000000000_diesel_initial_setup";
const DIESEL_UP: &str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/00000000000000_diesel_initial_setup/up.sql"
));

const DIESEL_DOWN: &str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/00000000000000_diesel_initial_setup/down.sql"
));

const SCHEMA_NAME: &str = "2024-08-12-093223_smdb";
const SCHEMA_UP: &str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/2024-08-12-093223_smdb/up.sql"
));
const SCHEMA_DOWN: &str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/2024-08-12-093223_smdb/down.sql"
));
