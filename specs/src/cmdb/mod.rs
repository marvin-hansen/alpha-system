use common::prelude::{
    Encoding, Endpoint, MainConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

pub fn cmdb_main_config() -> MainConfig {
    cmdb_service_config().main_config()
}

pub fn cmdb_service_config() -> ServiceConfig {
    let id = ServiceID::CMDB;
    let name = String::from("cmdbv1");
    let version = 1;
    let online = false;
    let description = String::from("CMDB Manages configurations stored in the DB");
    let health_check_uri = String::from("cmdb-service.default.svc.cluster.local:5050/health");
    let base_uri = String::from("cmdb-service.default.svc.cluster.local");
    let dependencies = vec![ServiceID::MEMGRAPH, ServiceID::SMDB];
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
    let endpoint_name = String::from("cmdb Endpoint");
    let endpoint_version = 1;
    let endpoint_description =
        String::from("Access to the configuration service via gRPC on baseUri:7070");
    let endpoint_uri = String::from("/");
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
