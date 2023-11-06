use common::prelude::{
    Encoding, Endpoint, MainConfig, Protocol, ServiceConfig, ServiceID, ServiceType,
};

pub fn smdb_main_config() -> MainConfig {
    smdb_service_config().get_main_config()
}

pub fn smdb_service_config() -> ServiceConfig {
    let id = ServiceID::SMDB;
    let name = String::from("smdbv1");
    let version = 1;
    let online = false;
    let description = String::from("SMDB Service Management Database");
    let health_check_uri = String::from("smdb-service.default.svc.cluster.local:5050/health");
    let base_uri = String::from("smdb-service.default.svc.cluster.local");
    let dependencies = vec![ServiceID::MEMGRAPH];
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
    let endpoint_name = String::from("service-registry");
    let endpoint_version = 1;
    let endpoint_description =
        String::from("Access to the SMDB service registry via gRPC on baseUri:5050");
    let endpoint_uri = String::from("/");
    let endpoint_port = 5050;
    let endpoint_protocol = Protocol::GRPC;
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
