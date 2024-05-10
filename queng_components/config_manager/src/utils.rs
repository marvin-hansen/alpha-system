use common::prelude::{ClickHouseConfig, EnvironmentType, ServiceConfig, ServiceID, SvcEnvConfig};
use db_specs::clickhouse;

pub(crate) fn get_db_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::LOCAL => clickhouse::get_local_db_config(),
        EnvironmentType::CLUSTER => clickhouse::get_cluster_db_config(),
        _ => ClickHouseConfig::default(),
    }
}

pub(crate) fn get_svc_env_config(
    service_id: ServiceID,
    service_config: &ServiceConfig,
) -> SvcEnvConfig {
    let binding = service_config.endpoint();
    let endpoint = binding.host_endpoint();

    let metrics_config = service_config.metrics();
    let local_host = "0.0.0.0".to_string();
    let cluster_host = endpoint.host_uri().to_string();
    let ci_host = "127.0.0.1".to_string();
    let docker_host = "0.0.0.0".to_string();
    let service_port = endpoint.port().to_string();
    let metrics_host = metrics_config.metric_host().to_string();
    let metrics_uri = metrics_config.metric_uri().to_string();
    let metrics_port = metrics_config.metric_port();

    SvcEnvConfig::new(
        service_id,
        cluster_host,
        ci_host,
        local_host,
        docker_host,
        service_port,
        metrics_host,
        metrics_uri,
        metrics_port,
    )
}
