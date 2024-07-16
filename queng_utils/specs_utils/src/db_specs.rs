use common_database::prelude::ClickHouseConfig;
use common_env::prelude::EnvironmentType;

pub fn get_clickhouse_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    db_specs::clickhouse_db::get_clickhouse_config(env_type)
}
