use common_config::prelude::{ProtocolType, ServiceID, ServiceType};
use qdgw_specs::qdgw_service_config;

#[test]
fn test_cmdb_service_config() {
    let service_config = qdgw_service_config();

    assert_eq!(service_config.svc_id(), &ServiceID::QDGW);
    assert_eq!(service_config.name(), "qdgwv1");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(
        service_config.description(),
        "QDGW gives access to quantitative Tick / min data"
    );
    assert_eq!(
        service_config.health_check_uri(),
        "qdgwv1-service.default.svc.cluster.local:7070/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "qdgwv1-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::SMDB].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "qdgw Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}
