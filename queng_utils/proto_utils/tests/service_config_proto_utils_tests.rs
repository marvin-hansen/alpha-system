use common_config::prelude::{Endpoint, ServiceConfig, ServiceID};
use proto_bindings::proto::{ProtoEndpoint, ProtoServiceConfig};
use proto_utils::service_config_proto_utils::{service_config_from_proto, service_config_to_proto};

#[test]
fn test_service_config_from_proto() {
    let proto = ProtoServiceConfig {
        svc_id: 1,
        name: "Test Service".to_string(),
        version: 1,
        online: true,
        description: "Test description".to_string(),
        health_check_uri: "/health".to_string(),
        base_uri: "http://localhost:8080".to_string(),
        dependencies: vec![2, 3],
        endpoint: Vec::from([ProtoEndpoint::default()]),
    };

    let config = service_config_from_proto(proto).unwrap();

    assert_eq!(config.svc_id(), &ServiceID::SMDB);
    assert_eq!(config.name(), "Test Service");
    assert_eq!(config.version(), 1);
    assert!(config.online());
    assert_eq!(config.description(), "Test description");
    assert_eq!(config.health_check_uri(), "/health");
    assert_eq!(config.base_uri(), "http://localhost:8080");
    assert_eq!(
        config.dependencies(),
        &vec![ServiceID::CMDB, ServiceID::DBGW]
    );
}

#[test]
fn test_to_proto() {
    let config = ServiceConfig::new(
        ServiceID::SMDB,
        "Test Service".to_string(),
        1,
        true,
        "Test description".to_string(),
        "/health".to_string(),
        "http://localhost:8080".to_string(),
        vec![ServiceID::CMDB, ServiceID::DBGW],
        Vec::from([Endpoint::default()]),
    );

    let proto = service_config_to_proto(config).unwrap();

    assert_eq!(proto.svc_id, 1);
    assert_eq!(proto.name, "Test Service");
    assert_eq!(proto.version, 1);
    assert!(proto.online);
    assert_eq!(proto.description, "Test description");
    assert_eq!(proto.health_check_uri, "/health");
    assert_eq!(proto.base_uri, "http://localhost:8080");
    assert_eq!(proto.dependencies, vec![2, 3]);
}
