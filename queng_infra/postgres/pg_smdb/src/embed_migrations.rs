use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};
use std::env;

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, SERVICE_MIGRATION]);

pub(crate) const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

pub const SERVICE_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    SERVICE_UP,
    Some(SERVICE_DOWN),
    EmbeddedName::new(SERVICE_NAME),
    TomlMetadataWrapper::new(true),
);

const DIESEL_NAME: &'static str = "00000000000000_diesel_initial_setup";
const DIESEL_UP: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/migrations/00000000000000_diesel_initial_setup/up.sql"
));

const DIESEL_DOWN: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/migrations/00000000000000_diesel_initial_setup/down.sql"
));

const SERVICE_NAME: &'static str = "2024-08-12-093223_smdb";
const SERVICE_UP: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/migrations/2024-08-12-093223_smdb/up.sql"
));
const SERVICE_DOWN: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/migrations/2024-08-12-093223_smdb/down.sql"
));

const MIGRATION_FOLDER: &'static str = {
    if option_env!("MIGRATION_DATA").is_some() {
        stringify!(env!("MIGRATION_DATA"))
    } else {
        "queng_infra/postgres/pg_smdb/migrations"
    }
};
