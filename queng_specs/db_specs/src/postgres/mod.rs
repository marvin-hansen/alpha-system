use common_database::prelude::PostgresDBConfig;
use common_env::prelude::EnvironmentType;

pub use crate::postgres;

pub fn get_postgres_config(env_type: &EnvironmentType) -> PostgresDBConfig {
    match env_type {
        EnvironmentType::LOCAL => get_local_db_config(),
        EnvironmentType::CLUSTER => get_cluster_db_config(),
        EnvironmentType::CI => get_ci_db_config(),
        _ => PostgresDBConfig::default(),
    }
}

pub fn get_local_db_config() -> PostgresDBConfig {
    get_base_postgres_db_config()
}

pub fn get_ci_db_config() -> PostgresDBConfig {
    get_base_postgres_db_config()
}

pub fn get_cluster_db_config() -> PostgresDBConfig {
    PostgresDBConfig::new(
        "http://postgres.default.svc.cluster.local".to_string(),
        "username".to_string(),
        "password".to_string(),
        "prod_db".to_string(),
        5432,
        5,
    )
}

pub(crate) fn get_base_postgres_db_config() -> PostgresDBConfig {
    PostgresDBConfig::new(
        "localhost".to_string(),
        "postgres".to_string(),
        "postgres".to_string(),
        "postgres".to_string(),
        5432,
        5,
    )
}
