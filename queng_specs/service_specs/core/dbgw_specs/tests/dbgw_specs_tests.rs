use common_config::prelude::{ProtocolType, ServiceID};
use dbgw_specs::dbgw_service_config;

#[test]
fn test_dbgw_service_config() {
    let service_config = dbgw_service_config();

    assert_eq!(service_config.svc_id(), &ServiceID::DBGW);
    assert_eq!(service_config.name(), "dbgw");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), "DBGW gives access to the DB");
    assert_eq!(
        service_config.health_check_uri(),
        "dbgw-service.default.svc.cluster.local:7070/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "dbgw-service.default.svc.cluster.local"
    );
    assert_eq!(service_config.dependencies().len(), 0);
    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "dbgw Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 9090);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}
