use common::prelude::{
    Encoding, Endpoint, ExchangeID, MetricConfig, ProtocolType, ServiceConfig, ServiceID,
    ServiceType,
};

pub(crate) fn ims_service_config(exchange_id: ExchangeID, service_id: ServiceID) -> ServiceConfig {
    let port = 7070;
    let id = service_id;
    let name = format!("ims-service-{}", exchange_id);
    let version = 1;
    let online = false;
    let description = format!("IMS controls streaming data for {} exchange", exchange_id);
    let health_check_uri = format!(
        "ims-data-service-{}.default.svc.cluster.local:{}/health",
        exchange_id, port
    );
    let base_uri = format!("ims-data-service-{}.default.svc.cluster.local", exchange_id);
    let dependencies = vec![ServiceID::SMDB];
    let exposure = ServiceType::ENDPOINT;
    let endpoint = get_endpoint(exchange_id, port);
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

fn get_endpoint(exchange_id: ExchangeID, port: u16) -> Endpoint {
    let endpoint_name = format!("{}-ims-data-endpoint", exchange_id);
    let endpoint_version = 1;
    let endpoint_description = format!(
        "Control {} exchange streaming data data via gRPC on baseUri:{}",
        exchange_id, port
    );
    let endpoint_uri = "/".to_string();
    let endpoint_port = port;
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
    let metric_port = 8080;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
