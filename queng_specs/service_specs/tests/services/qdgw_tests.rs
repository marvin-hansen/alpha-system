use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use service_specs::prelude::qdgw_service_config;

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
        "qdgw-service.default.svc.cluster.local:4040/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "qdgw-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::SMDB].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "qdgw Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to quantitative data via gRPC on baseUri:4040"
    );
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 4040);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}
