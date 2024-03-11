use common::prelude::SurrealDBConfig;

const PORT: u16 = 8000;

pub fn db_config_local() -> SurrealDBConfig {
    SurrealDBConfig::new_connection(PORT, "0.0.0.0".to_string())
}

pub fn db_config_ci() -> SurrealDBConfig {
    // Placebo config so that tests run in ci
    SurrealDBConfig::default()
}

pub fn db_config_cluster() -> SurrealDBConfig {
    SurrealDBConfig::new(
        PORT,
        "db.namespace.url.cluster".to_string(),
        "cluster".to_string(),
        "service".to_string(),
        "root".to_string(),
        "root".to_string(),
    )
}
