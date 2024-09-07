use common_database::prelude::{ClickHouseConfig, PostgresDBConfig};
use common_env::prelude::EnvironmentType;

pub fn get_clickhouse_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    db_specs::clickhouse::get_clickhouse_config(env_type)
}

pub fn get_postgres_config(env_type: &EnvironmentType) -> PostgresDBConfig {
    db_specs::postgres::get_postgres_config(env_type)
}
