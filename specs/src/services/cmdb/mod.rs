use common::prelude::{Encoding, Endpoint, MetricConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType};

pub fn cmdb_service_config() -> ServiceConfig {
    let id = ServiceID::CMDB;
    let name = "cmdbv1".to_string();
    let version = 1;
    let online = false;
    let description = "CMDB Manages configurations stored in the DB".to_string();
    let health_check_uri = "cmdb-service.default.svc.cluster.local:5050/health".to_string();
    let base_uri = "cmdb-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::DBGW, ServiceID::SMDB];
    let exposure = ServiceType::ENDPOINT;
    let endpoint = get_endpoint();
    let metrics = MetricConfig::default();

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
        metrics
    )
}

fn get_endpoint() -> Endpoint {
    let endpoint_name = "cmdb Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_description =
        "Access to the configuration service via gRPC on baseUri:8080".to_string();
    let endpoint_uri = "/".to_string();
    let endpoint_port = 8080;
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
