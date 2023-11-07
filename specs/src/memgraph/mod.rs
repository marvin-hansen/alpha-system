use common::prelude::{
    Encoding, Endpoint, MainConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

pub fn memgraph_main_config() -> MainConfig {
    memgraph_service_config().main_config()
}

pub fn memgraph_service_config() -> ServiceConfig {
    let id = ServiceID::MEMGRAPH;
    let name = String::from("memgraphv1");
    let version = 1;
    let online = false;
    let description = String::from("graph database service");
    let health_check_uri = String::from("memgraph-service.default.svc.cluster.local:5050/health");
    let base_uri = String::from("memgraph-service.default.svc.cluster.local");
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
    let endpoint_name = String::from("memgraph DB Endpoint");
    let endpoint_version = 1;
    let endpoint_description =
        String::from("Access to the memgraph via its client on baseUri:7687");
    let endpoint_uri = String::from("/");
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
