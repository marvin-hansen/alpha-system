use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use specs::prelude::smdb_service_config;

#[test]
fn test_smdb_main_config() {
    let service_config = smdb_service_config();
    let main_config = service_config.main_config();
    assert_eq!(main_config.id(), &ServiceID::SMDB);
    assert_eq!(main_config.name(), "smdbv1");
    assert_eq!(main_config.port(), 5050);
    assert_eq!(main_config.protocol(), &ProtocolType::GRPC);
}

#[test]
fn test_smdb_service_config() {
    let service_config = smdb_service_config();

    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), "smdbv1");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(
        service_config.description(),
        "SMDB Service Management Database"
    );
    assert_eq!(
        service_config.health_check_uri(),
        "smdb-service.default.svc.cluster.local:5050/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "smdb-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::MEMGRAPH].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "service-registry");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to the SMDB service registry via gRPC on baseUri:5050"
    );
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 5050);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}
