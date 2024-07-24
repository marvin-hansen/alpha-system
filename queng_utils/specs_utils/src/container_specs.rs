use common_container::prelude::ContainerConfig;
use container_specs::api_proxy_container_specs::api_proxy_container_config;
use container_specs::clickhouse_container_specs::clickhouse_container_config;
use container_specs::postgres_container_specs::postgres_db_container_config;

pub fn get_all_container_specs() -> Vec<ContainerConfig<'static>> {
    Vec::from([
        api_proxy_container_config(),
        postgres_db_container_config(),
        clickhouse_container_config(),
    ])
}

pub fn api_proxy_container_specs() -> ContainerConfig<'static> {
    api_proxy_container_config()
}

pub fn clickhouse_container_specs() -> ContainerConfig<'static> {
    clickhouse_container_config()
}

pub fn postgres_db_container_specs() -> ContainerConfig<'static> {
    postgres_db_container_config()
}
