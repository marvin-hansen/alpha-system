use common::prelude::{
    Encoding, Endpoint, MetricConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

pub fn smdb_service_config() -> ServiceConfig {
    let svc_id = ServiceID::SMDB;
    let name = "smdbv1".to_string();
    let version = 1;
    let online = false;
    let description = "SMDB Service Management Database".to_string();
    let health_check_uri = "smdb-service.default.svc.cluster.local:5050/health".to_string();
    let base_uri = "smdb-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let exposure = ServiceType::ENDPOINT;
    let endpoint = get_endpoint();
    let metrics = get_metric_config();

    ServiceConfig::new(
        svc_id,
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
    let endpoint_name = "service-registry".to_string();
    let endpoint_version = 1;
    let endpoint_description =
        "Access to the SMDB service registry via gRPC on baseUri:7070".to_string();
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
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    // Default port is 8080, but this would conflict with the default port of the metric endpoints
    // of  other services hence it is set to 8081.
    let metric_port = 8081;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
