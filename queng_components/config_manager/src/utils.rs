use common_config::prelude::{ServiceConfig, ServiceID, SvcEnvConfig};

pub(crate) fn get_svc_env_config(
    service_id: ServiceID,
    service_config: &ServiceConfig,
) -> SvcEnvConfig {
    let binding = service_config.service_endpoint();
    let endpoint = binding.host_endpoint();

    let metrics_config = service_config.metrics_endpoint();
    let local_host = "0.0.0.0".to_string();
    let cluster_host = endpoint.host_uri().to_string();
    let ci_host = "127.0.0.1".to_string();
    let docker_host = "0.0.0.0".to_string();
    let service_port = endpoint.port().to_string();
    let metrics_host = metrics_config.host().to_string();
    let metrics_uri = metrics_config.uri().to_string();
    let metrics_port = metrics_config.port();

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
