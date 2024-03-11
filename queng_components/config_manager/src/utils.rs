use common::prelude::{ClickHouseConfig, EnvironmentType};
use db_specs::clickhouse;

pub(crate) fn get_db_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::LOCAL => clickhouse::get_local_db_config(),
        EnvironmentType::CLUSTER => clickhouse::get_cluster_db_config(),
        _ => ClickHouseConfig::default(),
    }
}
