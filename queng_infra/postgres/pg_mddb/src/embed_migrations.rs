use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, MDDB_MIGRATION]);

const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

const MDDB_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    MDDB_UP,
    Some(MDDB_DOWN),
    EmbeddedName::new(MDDB_NAME),
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

const MDDB_NAME: &str = "2024-08-23-093731_mddb";

const MDDB_UP: &str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/2024-08-23-093731_mddb/up.sql"
));

const MDDB_DOWN: &str = include_str!(concat!(
    env!("MIGRATION_DATA"),
    "/2024-08-23-093731_mddb/down.sql"
));
