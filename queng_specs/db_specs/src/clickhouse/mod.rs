use common::prelude::ClickHouseConfig;

/// Retrieves the database configuration for a local environment.
///
/// This function fetches the necessary database connection details such as hostname,
/// port, username, password, and database name that are required to connect to the
/// database within a local environment.
///
/// # Returns
/// A `DbConfig` struct containing all the required fields to establish a database connection.
pub fn get_local_db_config() -> ClickHouseConfig {
    ClickHouseConfig::new(
        "127.0.0.1".to_string(),
        9000,
        "".to_string(),
        "".to_string(),
        "default".to_string(),
    )
}

/// Retrieves the database configuration for a cluster environment.
///
/// This function fetches the necessary database connection details such as hostname,
/// port, username, password, and database name that are required to connect to the
/// database within a cluster environment.
///
/// # Returns
/// A `DbConfig` struct containing all the required fields to establish a database connection.
pub fn get_cluster_db_config() -> ClickHouseConfig {
    ClickHouseConfig::new(
        "http://clickhouse.default.svc.cluster.local".to_string(),
        8123,
        "username".to_string(),
        "password".to_string(),
        "default".to_string(),
    )
}
