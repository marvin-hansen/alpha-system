use common::prelude::{
    Encoding, Endpoint, MetricConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

/// Returns a ServiceConfig for the SYMDB service.
///
/// # Returns
///
/// ServiceConfig with:
/// - id: ServiceID::SYMDB
/// - name: "symdbv1"
/// - version: 1
/// - online: false
/// - description: "SYMDB gives access to symbol metadata"
/// - health_check_uri: "health"
/// - local_host: "0.0.0.0"
/// - local_port: [7070, 8081]
/// - cluster_host: "symdb-service.default.svc.cluster.local"
/// - cluster_port: [7070, 8081]
/// - dependencies: None
/// - metrics: MetricConfig with custom port 8081 to avoid port clashes on localhost.
///
/// # Remarks
///
/// Used to configure the SYMDB service.
///
pub fn symdb_service_config() -> ServiceConfig {
    let id = ServiceID::SYMDB;
    let name = "symdbv1".to_string();
    let version = 1;
    let online = false;
    let description =
        "SYMDB (Symbol Master Database) gives access to central symbol to ID mapping)".to_string();
    let health_check_uri = "health".to_string();
    let base_uri = "symdb-service.default.svc.cluster.local".to_string();
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
    let endpoint_name = "symbol-master".to_string();
    let endpoint_version = 1;
    let endpoint_description =
        "Access to the SYMDB symbol mastger via gRPC on baseUri:7777".to_string();
    let endpoint_uri = "/".to_string();
    let endpoint_port = 7777;
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

/// Returns a MetricConfig for the SYMDB service
/// with a custom metric_port to avoid port clashes on localhost.
///
/// # Returns
///
/// MetricConfig with:
/// - metric_uri: "metrics"
/// - metric_host: "0.0.0.0"
/// - metric_port: 8081 (alternative prometheus port)
///
/// # Remarks
///
/// Used to configure prometheus metrics for the SYMDB service.
///
fn get_metric_config() -> MetricConfig {
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    // Default port for prometheus metrics is 8080
    let metric_port = 8085;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
