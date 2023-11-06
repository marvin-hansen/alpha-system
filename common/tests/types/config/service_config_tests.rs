use common::prelude::{Endpoint, ProtocolType, ServiceConfig, ServiceID, ServiceType};

#[test]
fn test_new() {
    let id = ServiceID::SMDB;
    let name = String::from("name");
    let version = 1;
    let online = true;
    let description = String::from("description");
    let health_check_uri = String::from("health_check_uri");
    let base_uri = String::from("base_uri");
    let dependencies = vec![ServiceID::MEMGRAPH];
    let exposure = ServiceType::default();
    let endpoint = Endpoint::default();

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
    );

    assert_eq!(service_config.id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), String::from("name"));
    assert_eq!(service_config.version(), 1);
    assert!(service_config.online());
    assert_eq!(service_config.description(), String::from("description"));
    assert_eq!(
        service_config.health_check_uri(),
        String::from("health_check_uri")
    );
    assert_eq!(service_config.base_uri(), String::from("base_uri"));
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::MEMGRAPH].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::default());
    assert_eq!(service_config.endpoint(), &Endpoint::default());
}

#[test]
fn test_get_main_config() {
    let id = ServiceID::SMDB;
    let name = ServiceID::SMDB.to_string();
    let version = 1;
    let online = true;
    let description = String::from("description");
    let health_check_uri = String::from("health_check_uri");
    let base_uri = String::from("base_uri");
    let dependencies = vec![ServiceID::MEMGRAPH];
    let exposure = ServiceType::default();
    let endpoint = Endpoint::default();

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
    );

    let main_config = service_config.get_main_config();

    assert_eq!(main_config.id(), &ServiceID::SMDB);
    assert_eq!(main_config.name(), String::from("SMDB"));
    assert_eq!(main_config.port(), 0);
    assert_eq!(main_config.protocol(), &ProtocolType::GRPC);
}

#[test]
fn test_default() {
    let service_config = ServiceConfig::default();

    assert_eq!(service_config.id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), &String::from(""));
    assert_eq!(service_config.version(), 0);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), &String::from(""));
    assert_eq!(service_config.health_check_uri(), &String::from(""));
    assert_eq!(service_config.base_uri(), &String::from(""));
    assert_eq!(service_config.dependencies(), &Vec::new());
    assert_eq!(service_config.exposure(), &ServiceType::default());
    assert_eq!(service_config.endpoint(), &Endpoint::default());
}

#[test]
fn test_display() {
    let id = ServiceID::SMDB;
    let name = ServiceID::SMDB.to_string();
    let version = 1;
    let online = true;
    let description = String::from("description");
    let health_check_uri = String::from("health_check_uri");
    let base_uri = String::from("base_uri");
    let dependencies = vec![ServiceID::MEMGRAPH];
    let exposure = ServiceType::default();
    let endpoint = Endpoint::default();

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
    );

    let expected = "ServiceConfig { id: SMDB, name: SMDB, version: 1, online: true, description: description, health_check_uri: health_check_uri, base_uri: base_uri, dependencies: [MEMGRAPH], exposure: ENDPOINT, endpoint: name: ,  version: 0,  port: 0,  description: ,  uri: ,  protocol: GRPC,  encoding: Protobuf }";
    let actual = service_config.to_string();
    assert_eq!(actual, expected);
}
