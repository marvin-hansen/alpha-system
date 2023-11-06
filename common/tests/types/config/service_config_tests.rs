use common::types::config::endpoint::Endpoint;
use common::types::config::protocol::Protocol;
use common::types::config::service_config::ServiceConfig;
use common::types::config::service_name::ServiceName;
use common::types::config::service_type::ServiceType;

#[test]
fn test_new() {
    let id = String::from("id");
    let name = String::from("name");
    let version = 1;
    let online = true;
    let description = String::from("description");
    let health_check_uri = String::from("health_check_uri");
    let base_uri = String::from("base_uri");
    let dependencies = vec![ServiceName::default()];
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

    assert_eq!(service_config.id(), String::from("id"));
    assert_eq!(service_config.name(), String::from("name"));
    assert_eq!(service_config.version(), 1);
    assert_eq!(service_config.online(), true);
    assert_eq!(service_config.description(), String::from("description"));
    assert_eq!(service_config.health_check_uri(), String::from("health_check_uri"));
    assert_eq!(service_config.base_uri(), String::from("base_uri"));
    assert_eq!(service_config.dependencies().len(), vec![ServiceName::default()].len());
    assert_eq!(service_config.exposure(), &ServiceType::default());
    assert_eq!(service_config.endpoint(), &Endpoint::default());
}

#[test]
fn test_get_main_config() {
    let id = String::from("id");
    let name = String::from("name");
    let version = 1;
    let online = true;
    let description = String::from("description");
    let health_check_uri = String::from("health_check_uri");
    let base_uri = String::from("base_uri");
    let dependencies = vec![ServiceName::default()];
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

    assert_eq!(main_config.id(), String::from("id"));
    assert_eq!(main_config.name(), String::from("name"));
    assert_eq!(main_config.port(), 0);
    assert_eq!(main_config.protocol(), &Protocol::GRPC);
}


#[test]
fn test_default() {
    let service_config = ServiceConfig::default();

    assert_eq!(service_config.id(), &String::from(""));
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
    let id = String::from("id");
    let name = String::from("name");
    let version = 1;
    let online = true;
    let description = String::from("description");
    let health_check_uri = String::from("health_check_uri");
    let base_uri = String::from("base_uri");
    let dependencies = vec![ServiceName::default()];
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
    let expected = "ServiceConfig { id: id, name: name, version: 1, online: true, description: description, health_check_uri: health_check_uri, base_uri: base_uri, dependencies: [UNKNOWN], exposure: ENDPOINT, endpoint: endpoint_name: , endpoint_version: 0, endpoint_port: 0, endpoint_description: , endpoint_uri: , endpoint_protocol: GRPC, endpoint_encoding: Protobuf }";
    assert_eq!(service_config.to_string(), expected);
}
