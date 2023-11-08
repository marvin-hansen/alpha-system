use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use specs::prelude::{memgraph_main_config, memgraph_service_config};

#[test]
fn test_memgraph_main_config() {
    let main_config = memgraph_main_config();
    assert_eq!(main_config.id(), &ServiceID::MEMGRAPH);
    assert_eq!(main_config.name(), "memgraphv1");
    assert_eq!(main_config.port(), 7687);
    assert_eq!(main_config.protocol(), &ProtocolType::HTTP);
}

#[test]
fn test_memgraph_service_config() {
    let service_config = memgraph_service_config();

    assert_eq!(service_config.id(), &ServiceID::MEMGRAPH);
    assert_eq!(service_config.name(), "memgraphv1");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), "graph database service");
    assert_eq!(
        service_config.health_check_uri(),
        "memgraph-service.default.svc.cluster.local:5050/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "memgraph-service.default.svc.cluster.local"
    );
    assert_eq!(service_config.dependencies().len(), 0);
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "memgraph DB Endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to the memgraph via its client on baseUri:7687"
    );
    assert_eq!(endpoint.uri(), String::from("/"));
    assert_eq!(endpoint.port(), 7687);
    assert_eq!(endpoint.protocol(), &ProtocolType::HTTP);
    assert_eq!(endpoint.encoding(), &Encoding::Binary);
}
