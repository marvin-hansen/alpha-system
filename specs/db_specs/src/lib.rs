pub mod prelude;

use common::prelude::DBConfig;

const PORT: u16 = 8000;

pub fn db_config_local() -> DBConfig {
    DBConfig::new_connection(PORT, "0.0.0.0".to_string())
}

pub fn db_config_ci() -> DBConfig {
    // Placebo config so that tests run in ci
    DBConfig::default()
}

pub fn db_config_cluster() -> DBConfig {
    DBConfig::new(
        PORT,
        "db.namespace.url.cluster".to_string(),
        "cluster".to_string(),
        "service".to_string(),
        "root".to_string(),
        "root".to_string(),
        String::from("dbgw"),
    )
}
