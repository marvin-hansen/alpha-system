use common_config::prelude::ContainerConfig;

/// Constructs the configuration for a ClickHouse container.
///
/// This function prepares a `ContainerConfig` specifically tailored for the deployment
/// of a Clickhouse database container.
///
/// # Returns
/// A `ContainerConfig` instance populated with all the necessary settings for a ClickHouse container.
pub fn clickhouse_container_config() -> ContainerConfig<'static> {
    // Official container image for ClickHouse
    // https://hub.docker.com/r/clickhouse/clickhouse-server/tags
    ContainerConfig::new(
        "clickhouse",
        "clickhouse/clickhouse-server",
        "24.6.1",
        "0.0.0.0",
        9000,
        Some(&[8123]),
        None,
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        5,    // Wait 5 second until the container finished starting up.
    )
}
