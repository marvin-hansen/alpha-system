use crate::surreal_db::ci::surreal_db_ci_config;
use crate::surreal_db::cluster::surreal_db_cluster_config;
use crate::surreal_db::local::surreal_db_local_config;
use common_database::prelude::SurrealDBConfig;
use common_env::prelude::EnvironmentType;

pub mod ci;
pub mod cluster;
pub mod local;

pub fn get_surreal_config(env_type: &EnvironmentType) -> SurrealDBConfig {
    match env_type {
        EnvironmentType::LOCAL => surreal_db_local_config::get_local_surreal_db_config(),
        EnvironmentType::CI => surreal_db_ci_config::get_ci_surreal_db_config(),
        EnvironmentType::CLUSTER => surreal_db_cluster_config::get_cluster_surreal_db_config(),
        _ => SurrealDBConfig::default(),
    }
}
