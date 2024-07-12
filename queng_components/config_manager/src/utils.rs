use common::prelude::{
    ClickHouseConfig, EnvironmentType, ServiceConfig, ServiceID, SurrealDBConfig, SvcEnvConfig,
};
use db_specs::prelude::{
    clickhouse_ci_config, clickhouse_cluster_config, clickhouse_local_config, surreal_db_ci_config,
    surreal_db_cluster_config, surreal_db_local_config,
};

pub(crate) fn get_db_specs_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::LOCAL => clickhouse_local_config::get_local_specs_db_config(),
        EnvironmentType::CI => clickhouse_ci_config::get_ci_specs_db_config(),
        EnvironmentType::CLUSTER => clickhouse_cluster_config::get_cluster_specs_db_config(),
        _ => ClickHouseConfig::default(),
    }
}

pub(crate) fn get_db_metadata_config(env_type: &EnvironmentType) -> ClickHouseConfig {
    match env_type {
        EnvironmentType::LOCAL => clickhouse_local_config::get_local_metadata_db_config(),
        EnvironmentType::CI => clickhouse_ci_config::get_ci_metadata_db_config(),
        EnvironmentType::CLUSTER => clickhouse_cluster_config::get_cluster_metadata_db_config(),
        _ => ClickHouseConfig::default(),
    }
}

pub(crate) fn get_db_surreal_config(env_type: &EnvironmentType) -> SurrealDBConfig {
    match env_type {
        EnvironmentType::LOCAL => surreal_db_local_config::get_local_surreal_db_config(),
        EnvironmentType::CI => surreal_db_ci_config::get_ci_surreal_db_config(),
        EnvironmentType::CLUSTER => surreal_db_cluster_config::get_cluster_surreal_db_config(),
        _ => SurrealDBConfig::default(),
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
