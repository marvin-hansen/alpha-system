use common::prelude::DBConfig;

const PORT: u16 = 8000;

pub fn db_config_local() -> DBConfig {
    DBConfig::new_connection(PORT, "localhost".to_string())
}

pub fn db_config_ci() -> DBConfig {
    // Placebo config so that build and tests run in ci
    DBConfig::default()
}

pub fn db_config_cluster() -> DBConfig {
    DBConfig::new(
        PORT,
        "db.namespace.url,cluster".to_string(),
        "cluster".to_string(),
        "service".to_string(),
        "admin".to_string(),
        "password".to_string(),
        String::from("dbgw"),
    )
}
