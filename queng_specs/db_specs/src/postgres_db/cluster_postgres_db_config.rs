use common_database::prelude::PostgresDBConfig;

pub fn get_cluster_db_config() -> PostgresDBConfig {
    PostgresDBConfig::new(
        "http://postgres.default.svc.cluster.local".to_string(),
        "username".to_string(),
        "password".to_string(),
        "specs".to_string(),
        5432,
        5,
    )
}
