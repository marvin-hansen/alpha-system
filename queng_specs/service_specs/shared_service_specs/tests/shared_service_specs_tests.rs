use common_config::ProtocolType;
use shared_service_specs::health_endpoint;

#[test]
fn test_ims_endpoint() {
    let endpoint = shared_service_specs::ims_endpoint("exchange_id", 7070);
    assert_eq!(endpoint.name(), "exchange_id-ims-data-endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}
#[test]
fn test_default_grpc_service_endpoint() {
    let endpoint = shared_service_specs::default_grpc_service_endpoint("endpoint_name", 7070);
    assert_eq!(endpoint.name(), "endpoint_name");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}

#[test]
fn test_default_http_service_endpoint() {
    let endpoint =
        shared_service_specs::default_http_service_endpoint("endpoint_name", "endpoint_uri");
    assert_eq!(endpoint.name(), "endpoint_name");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "endpoint_uri");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::HTTP);
}

#[test]
fn test_metric_endpoint() {
    let endpoint = shared_service_specs::metric_endpoint();
    assert_eq!(endpoint.name(), "Metrics Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "metrics");
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), ProtocolType::HTTP);
}

#[test]
fn test_health_endpoint() {
    let endpoint = health_endpoint();
    assert_eq!(endpoint.name(), "Health Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "health");
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), ProtocolType::HTTP);
}
