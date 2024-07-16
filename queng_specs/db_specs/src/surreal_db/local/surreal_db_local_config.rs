use common_database::prelude::SurrealDBConfig;

pub fn get_local_surreal_db_config() -> SurrealDBConfig {
    SurrealDBConfig::default()
}
