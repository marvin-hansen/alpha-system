use cmdb_specs::cmdb_service_config;
use common_config::prelude::{ProtocolType, ServiceID};

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
        "cmdbv1-service.default.svc.cluster.local:8080/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "cmdbv1-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::DBGW, ServiceID::SMDB].len()
    );
    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "cmdb Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);

    let endpoint = service_config.metrics_endpoint();
    assert_eq!(endpoint.uri(), String::from("metrics"));
    assert_eq!(endpoint.host(), String::from("0.0.0.0"));
    assert_eq!(endpoint.port(), 8080);
}
