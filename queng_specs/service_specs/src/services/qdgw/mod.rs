use common::prelude::{
    Encoding, Endpoint, MetricConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

pub fn qdgw_service_config() -> ServiceConfig {
    let id = ServiceID::QDGW;
    let name = "qdgwv1".to_string();
    let version = 1;
    let online = false;
    let description = "QDGW gives access to quantitative Tick / min data".to_string();
    let health_check_uri = "qdgwv1-service.default.svc.cluster.local:5050/health".to_string();
    let base_uri = "qdgwv1-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::SMDB];
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
    let endpoint_name = "qdgw Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_description = "Access to quantitative data via gRPC on baseUri:4040".to_string();
    let endpoint_uri = "/".to_string();
    let endpoint_port = 5050;
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
