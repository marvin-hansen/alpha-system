use crate::prelude::ContainerConfig;

pub(crate) fn clickhouse_container_config() -> ContainerConfig<'static> {
    // Official container image for ClickHouse
    // https://hub.docker.com/r/clickhouse/clickhouse-server/tags
    ContainerConfig::new(
        "clickhouse",
        "clickhouse/clickhouse-server",
        "22.3.18",
        "0.0.0.0",
        9000,
        &[8123],
        true,  // Keep the container running for re-use
        false, // Keep the same container config across all env. setups.
    )
}
