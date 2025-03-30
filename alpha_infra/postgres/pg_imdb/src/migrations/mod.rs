/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod diesel_initial_setup;
mod imdb_setup;

use crate::migrations::diesel_initial_setup::down::DIESEL_DOWN;
use crate::migrations::diesel_initial_setup::up::DIESEL_UP;
use diesel_migrations::{EmbeddedMigration, EmbeddedMigrations, EmbeddedName, TomlMetadataWrapper};

use crate::migrations::imdb_setup::down::SCHEMA_DOWN;
use crate::migrations::imdb_setup::up::SCHEMA_UP;

pub const EMBEDDED_MIGRATIONS: EmbeddedMigrations =
    EmbeddedMigrations::new(&[DIESEL_MIGRATION, SCHEMA_MIGRATION]);

const DIESEL_NAME: &str = "00000000000000_diesel_initial_setup";
const DIESEL_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    DIESEL_UP,
    Some(DIESEL_DOWN),
    EmbeddedName::new(DIESEL_NAME),
    TomlMetadataWrapper::new(true),
);

const SCHEMA_NAME: &str = "2024-09-05-041426_imddb";
const SCHEMA_MIGRATION: EmbeddedMigration = EmbeddedMigration::new(
    SCHEMA_UP,
    Some(SCHEMA_DOWN),
    EmbeddedName::new(SCHEMA_NAME),
    TomlMetadataWrapper::new(true),
);
