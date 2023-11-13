use common::prelude::{Encoding, Endpoint, ProtocolType, ServiceConfig, ServiceID, ServiceType};

pub fn memgraph_service_config() -> ServiceConfig {
    let id = ServiceID::MEMGRAPH;
    let name = "memgraphv1".to_string();
    let version = 1;
    let online = false;
    let description = "graph database service".to_string();
    let health_check_uri = "memgraph-service.default.svc.cluster.local:5050/health".to_string();
    let base_uri = "memgraph-service.default.svc.cluster.local".to_string();
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
    let endpoint_name = "memgraph DB Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_description = "Access to the memgraph via its client on baseUri:7687".to_string();
    let endpoint_uri = "/".to_string();
    let endpoint_port = 7687;
    let endpoint_protocol = ProtocolType::HTTP;
    let endpoint_encoding = Encoding::Binary;

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
