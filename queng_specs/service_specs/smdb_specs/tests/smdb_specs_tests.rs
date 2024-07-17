use common_config::prelude::{ProtocolType, ServiceID, ServiceType};
use smdb_specs::smdb_service_config;

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
        "smdbv1-service.default.svc.cluster.local:7070/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "smdbv1-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::DBGW].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "service-registry");
    assert_eq!(endpoint.version(), 1);

    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}
