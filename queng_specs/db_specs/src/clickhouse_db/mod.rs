use crate::clickhouse_db::ci::clickhouse_ci_config;
use crate::clickhouse_db::cluster::clickhouse_cluster_config;
use crate::clickhouse_db::local::clickhouse_local_config;
use common_database::prelude::ClickHouseConfig;
use common_env::prelude::EnvironmentType;

pub mod ci;
pub mod cluster;
pub mod local;

pub fn get_clickhouse_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::LOCAL => clickhouse_local_config::get_local_metadata_db_config(),
        EnvironmentType::CI => clickhouse_ci_config::get_ci_metadata_db_config(),
        EnvironmentType::CLUSTER => clickhouse_cluster_config::get_cluster_metadata_db_config(),
        _ => ClickHouseConfig::default(),
    }
}
