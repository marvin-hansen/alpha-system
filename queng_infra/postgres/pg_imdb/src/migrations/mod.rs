pub(crate) mod diesel_initial_setup;
pub(crate) mod imdb_setup;

pub(crate) mod embed_migrations;

pub(crate) const DIESEL_NAME: &str = "00000000000000_diesel_initial_setup";
pub(crate) use crate::migrations::diesel_initial_setup::down::DIESEL_DOWN;
pub(crate) use crate::migrations::diesel_initial_setup::up::DIESEL_UP;

pub(crate) const SCHEMA_NAME: &str = "2024-09-05-041426_imddb";
pub(crate) use crate::migrations::imdb_setup::down::SCHEMA_DOWN;
pub(crate) use crate::migrations::imdb_setup::up::SCHEMA_UP;
