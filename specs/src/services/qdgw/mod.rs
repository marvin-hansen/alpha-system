use common::prelude::{Encoding, Endpoint, ProtocolType, ServiceConfig, ServiceID, ServiceType};

pub fn qdgw_service_config() -> ServiceConfig {
    let id = ServiceID::QDGW;
    let name = "qdgwv1".to_string();
    let version = 1;
    let online = true;
    let description = " QDGW gives access to quantitative Tick / min data".to_string();
    let health_check_uri = "qdg-service.default.svc.cluster.local:4040/health".to_string();
    let base_uri = "qdg-service.default.svc.cluster.local".to_string();
    let dependencies = vec![];
    let exposure = ServiceType::ENDPOINT;
    let endpoint = get_endpoint();

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
    )
}

fn get_endpoint() -> Endpoint {
    let endpoint_name = "qdg Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_description = "Access to quantitative data via gRPC on baseUri:4040".to_string();
    let endpoint_uri = "/".to_string();
    let endpoint_port = 4040;
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
