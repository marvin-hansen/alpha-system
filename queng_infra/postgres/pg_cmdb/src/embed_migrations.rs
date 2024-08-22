use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, CMDB_MIGRATION]);

pub(crate) const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

pub(crate) const CMDB_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    CMDB_UP,
    Some(CMDB_DOWN),
    EmbeddedName::new(CMDB_NAME),
    TomlMetadataWrapper::new(true),
);

const DIESEL_NAME: &'static str = "00000000000000_diesel_initial_setup";
const DIESEL_UP: &'static str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/00000000000000_diesel_initial_setup/up.sql"
));

const DIESEL_DOWN: &'static str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/00000000000000_diesel_initial_setup/down.sql"
));

const CMDB_NAME: &'static str = "2024-08-12-083114_cmdb";

const CMDB_UP: &'static str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/2024-08-12-083114_cmdb/up.sql"
));

const CMDB_DOWN: &'static str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/2024-08-12-083114_cmdb/down.sql"
));
