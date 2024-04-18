use crate::prelude::ContainerConfig;

pub(crate) fn clickhouse_container_config(reuse_container: bool) -> ContainerConfig<'static> {
    ContainerConfig::new(
        "clickhouse",
        "bitnami/clickhouse",
        "24.3.2-debian-12-r2",
        9000,
        reuse_container,
    )
}
