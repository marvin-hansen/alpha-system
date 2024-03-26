use common::prelude::ServiceConfig;
pub(crate) fn generate_service_insert(service_config: ServiceConfig) -> String {
    let table_name = "default.services";
    let svc_id = service_config.svc_id();
    let name = service_config.name();
    let version = service_config.version();
    let online = service_config.online();
    let description = service_config.description();
    let health_check_uri = service_config.health_check_uri();
    let base_uri = service_config.base_uri();
    let dependencies = service_config.dependencies().as_slice();
    let exposure = service_config.exposure();
    let endpoint = service_config.endpoint();

    format!(
        r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{svc_id}',
        '{name}',
        {version},
        {online},
        '{description}',
        '{health_check_uri}',
        '{base_uri}',
        '{dependencies}',
        '{exposure}',
        '{endpoint}'
        )
    "
    )
}

pub(crate) fn generate_count_services() -> String {
    r"
    SELECT count(*) FROM system.services
    "
    .to_string()
}
