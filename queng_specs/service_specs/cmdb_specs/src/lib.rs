use common_config::prelude::{
    Encoding, Endpoint, MetricConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

/// Constructs the configuration for the CMDB service.
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
pub fn cmdb_service_config() -> ServiceConfig {
    let id = ServiceID::CMDB;
    let name = "cmdbv1".to_string();
    let version = 1;
    let online = false;
    let description = "CMDB Manages configurations stored in the DB".to_string();
    let health_check_uri = "cmdbv1-service.default.svc.cluster.local:7070/health".to_string();
    let base_uri = "cmdbv1-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::DBGW, ServiceID::SMDB];
    let exposure = ServiceType::ENDPOINT;
    let endpoint = get_endpoint();
    let metrics = get_metric_config();

    ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
        metrics,
    )
}

fn get_endpoint() -> Endpoint {
    let endpoint_name = "cmdb Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_description =
        "Access to the service configuration service via gRPC on baseUri:7070".to_string();
    let endpoint_uri = "/".to_string();
    let endpoint_port = 7070;
    let endpoint_protocol = ProtocolType::GRPC;
    let endpoint_encoding = Encoding::Protobuf;

    Endpoint::new(
        endpoint_name,
        endpoint_version,
        endpoint_description,
        endpoint_uri,
        endpoint_port,
        endpoint_protocol,
        endpoint_encoding,
    )
}

fn get_metric_config() -> MetricConfig {
    let metric_uri = "metrics".to_string();
    let metric_host = "0.0.0.0".to_string();
    let metric_port = 8080;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
