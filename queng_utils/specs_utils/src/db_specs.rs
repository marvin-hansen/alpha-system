use common_database::prelude::{ClickHouseConfig, PostgresDBConfig};
use common_env::prelude::EnvironmentType;

pub fn get_clickhouse_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    db_specs::clickhouse_db::get_clickhouse_config(env_type)
}

pub fn get_postgres_config(env_type: &EnvironmentType) -> PostgresDBConfig {
    db_specs::postgres_db::get_postgres_config(env_type)
}
