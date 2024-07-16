mod ci_postgres_db_config;
mod cluster_postgres_db_config;
mod local_postgres_db_config;
mod shared_config;

use common_database::prelude::PostgresDBConfig;
use common_env::prelude::EnvironmentType;

pub fn get_postgres_config(env_type: &EnvironmentType) -> PostgresDBConfig {
    match env_type {
        EnvironmentType::LOCAL => local_postgres_db_config::get_local_db_config(),
        EnvironmentType::CLUSTER => cluster_postgres_db_config::get_cluster_db_config(),
        EnvironmentType::CI => ci_postgres_db_config::get_ci_db_config(),
        _ => PostgresDBConfig::default(),
    }
}
