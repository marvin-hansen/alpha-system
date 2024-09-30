pub(crate) mod cmdb_setup;
pub(crate) mod diesel_initial_setup;
pub(crate) mod embed_migrations;

pub const DIESEL_NAME: &str = "00000000000000_diesel_initial_setup";
pub use crate::migrations::diesel_initial_setup::down::DIESEL_DOWN;
pub use crate::migrations::diesel_initial_setup::up::DIESEL_UP;

pub const SCHEMA_NAME: &str = "2024-08-12-083114_cmdb";
pub use crate::migrations::cmdb_setup::down::SCHEMA_DOWN;
pub use crate::migrations::cmdb_setup::up::SCHEMA_UP;
