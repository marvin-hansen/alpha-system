use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use specs::prelude::cmdb_service_config;

#[test]
fn test_smdb_main_config() {
    let service_config = cmdb_service_config();

    let main_config = service_config.main_config();
    assert_eq!(main_config.id(), &ServiceID::CMDB);
    assert_eq!(main_config.name(), "cmdbv1");
    assert_eq!(main_config.port(), 7070);
    assert_eq!(main_config.protocol(), &ProtocolType::GRPC);
}

#[test]
fn test_cmdb_service_config() {
    let service_config = cmdb_service_config();

    assert_eq!(service_config.svc_id(), &ServiceID::CMDB);
    assert_eq!(service_config.name(), "cmdbv1");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(
        service_config.description(),
        "CMDB Manages configurations stored in the DB"
    );
    assert_eq!(
        service_config.health_check_uri(),
        "cmdb-service.default.svc.cluster.local:5050/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "cmdb-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::MEMGRAPH, ServiceID::SMDB].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "cmdb Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to the configuration service via gRPC on baseUri:7070"
    );
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}
