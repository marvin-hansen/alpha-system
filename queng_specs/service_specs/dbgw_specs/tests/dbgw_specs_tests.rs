use common_config::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use dbgw_specs::dbgw_service_config;

#[test]
fn test_cmdb_service_config() {
    let service_config = dbgw_service_config();

    assert_eq!(service_config.svc_id(), &ServiceID::DBGW);
    assert_eq!(service_config.name(), "dbgwv1");
    assert_eq!(service_config.version(), 1);
    assert!(service_config.online());
    assert_eq!(service_config.description(), "DBGW gives access to the DB");
    assert_eq!(
        service_config.health_check_uri(),
        "dbgwv1-service.default.svc.cluster.local:7070/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "dbgwv1-service.default.svc.cluster.local"
    );
    assert_eq!(service_config.dependencies().len(), 0);
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "dbgw Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to surreal DB via gRPC on baseUri:7070"
    );
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 9090);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}
