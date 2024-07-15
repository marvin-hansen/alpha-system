use common_config::prelude::{ClickHouseConfig, SurrealDBConfig};
use common_env::prelude::EnvironmentType;

pub fn get_clickhouse_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    db_specs::clickhouse_db::get_clickhouse_config(env_type)
}

pub fn get_surreal_config(env_type: &EnvironmentType) -> SurrealDBConfig {
    db_specs::surreal_db::get_surreal_config(env_type)
}
