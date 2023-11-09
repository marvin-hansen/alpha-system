use common::prelude::{Encoding, Endpoint, ProtocolType, ServiceConfig, ServiceID, ServiceType};

pub fn smdb_service_config<'l>() -> ServiceConfig<'l> {
    let id = ServiceID::SMDB;
    let name = "smdbv1";
    let version = 1;
    let online = false;
    let description = "SMDB Service Management Database";
    let health_check_uri = "smdb-service.default.svc.cluster.local:5050/health";
    let base_uri = "smdb-service.default.svc.cluster.local";
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

fn get_endpoint<'l>() -> Endpoint<'l> {
    let endpoint_name = "service-registry";
    let endpoint_version = 1;
    let endpoint_description = "Access to the SMDB service registry via gRPC on baseUri:5050";
    let endpoint_uri = "/";
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
