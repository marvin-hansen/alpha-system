use common_config::prelude::{ServiceConfig, ServiceID};
use shared_service_specs::{health_endpoint, ims_endpoint, metric_endpoint};

/// Constructs the configuration for the VEX service.
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
pub fn vex_service_config() -> ServiceConfig {
    let id = ServiceID::VEX;
    let name = "vexv1".to_string();
    let version = 1;
    let online = false;
    let description = "vex: Virtual Exchange".to_string();
    let health_check_uri = "vex-service.default.svc.cluster.local:9999/health".to_string();
    let base_uri = "vex-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::SMDB];
    let endpoints = vec![
        ims_endpoint(id.to_string().to_lowercase().as_str(), 7070),
        metric_endpoint(),
        health_endpoint(),
    ];

    ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        endpoints,
    )
}
