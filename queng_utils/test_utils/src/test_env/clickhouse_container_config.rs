use crate::prelude::ContainerConfig;

pub(crate) fn clickhouse_container_config() -> ContainerConfig<'static> {
    // Official container image for ClickHouse
    // https://hub.docker.com/r/clickhouse/clickhouse-server/tags
    ContainerConfig::new(
        "clickhouse",
        "clickhouse/clickhouse-server",
        "24.3.2",
        9000,
        false,
    )
}
