use crate::postgres_db::shared_config::get_base_postgres_db_config;
use common_database::prelude::PostgresDBConfig;

pub fn get_ci_db_config() -> PostgresDBConfig {
    get_base_postgres_db_config()
}
