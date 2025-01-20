use common_config::{ServiceConfig, ServiceID};
use shared_service_specs::{default_grpc_service_endpoint, health_endpoint, metric_endpoint};

/// Constructs the configuration for the SMDB service.
///
/// This function generates a `ServiceConfig` struct with the following fields:
/// - `id`: The unique identifier for the service.
/// - `name`: The human-readable name of the service.
/// - `version`: The version of the service.
/// - `online`: A boolean indicating whether the service is online or not.
/// - `description`: A brief description of the service.
/// - `health_check_uri`: The URI for the health check endpoint of the service.
/// - `base_uri`: The base URI for the service.
/// - `dependencies`: A list of service IDs that this service depends on.
/// - `exposure_type`: The exposure type of the service (e.g., internal, external).
/// - `endpoints`: A list of `EndpointConfig` structs defining the endpoints of the service.
/// - `metrics`: A list of `MetricConfig` structs defining the metrics of the service.
///
/// # Returns
/// A `ServiceConfig` instance with all the necessary settings for the CMDB service.
#[must_use]
pub fn smdb_service_config() -> ServiceConfig {
    let svc_id = ServiceID::SMDB;
    let name = "smdb".to_string();
    let version = 1;
    let online = false;
    let description = "SMDB Service Management Database".to_string();
    let health_check_uri = "smdb-service.default.svc.cluster.local:7070/health".to_string();
    let cluster_uri = "smdb-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let endpoints = vec![
        default_grpc_service_endpoint("SMDB Endpoint", 7070),
        metric_endpoint(),
        health_endpoint(),
    ];

    ServiceConfig::new(
        svc_id,
        name,
        version,
        online,
        description,
        health_check_uri,
        cluster_uri,
        dependencies,
        endpoints,
    )
}
