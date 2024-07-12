use common::prelude::ContainerConfig;
use container_specs::api_proxy_container_specs::api_proxy_container_config;
use container_specs::clickhouse_container_specs::clickhouse_container_config;
use container_specs::surreal_db_specs::surreal_db_container_config;

pub fn get_all_container_specs() -> Vec<ContainerConfig<'static>> {
    Vec::from([
        api_proxy_container_config(),
        surreal_db_container_config(),
        clickhouse_container_config(),
    ])
}

pub fn api_proxy_container_specs() -> ContainerConfig<'static> {
    api_proxy_container_config()
}

pub fn clickhouse_container_specs() -> ContainerConfig<'static> {
    clickhouse_container_config()
}

pub fn surreal_db_container_specs() -> ContainerConfig<'static> {
    surreal_db_container_config()
}
