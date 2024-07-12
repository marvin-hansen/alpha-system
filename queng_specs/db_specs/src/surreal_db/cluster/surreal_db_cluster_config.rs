use common::prelude::SurrealDBConfig;

pub fn get_cluster_surreal_db_config() -> SurrealDBConfig {
    SurrealDBConfig::new(
        8000,
        "http://surrealdb.default.svc.cluster.local".to_string(),
        "db_name".to_string(),
        "db_namespace".to_string(),
        "db_username".to_string(),
        "db_password".to_string(),
    )
}
