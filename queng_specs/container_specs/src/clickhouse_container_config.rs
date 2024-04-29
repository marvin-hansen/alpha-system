use common::prelude::ContainerConfig;

pub fn clickhouse_container_config() -> ContainerConfig<'static> {
    // Official container image for ClickHouse
    // https://hub.docker.com/r/clickhouse/clickhouse-server/tags
    ContainerConfig::new(
        "clickhouse",
        "clickhouse/clickhouse-server",
        "24.3.2",
        "127.0.0.1",
        9000,
        &[8123],
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
    )
}
