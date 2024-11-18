use common_database::ClickHouseConfig;
use common_env::EnvironmentType;

pub mod ci_clickhouse_config;
pub mod cluster_clickhouse_config;
pub mod local_clickhouse_config;

pub fn get_clickhouse_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::LOCAL => local_clickhouse_config::get_local_db_config(),
        EnvironmentType::CI => ci_clickhouse_config::get_ci_db_config(),
        EnvironmentType::CLUSTER => cluster_clickhouse_config::get_cluster_db_config(),
        _ => ClickHouseConfig::default(),
    }
}
