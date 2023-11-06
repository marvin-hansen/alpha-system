use common::prelude::{MainConfig, ProtocolType, ServiceID};

#[test]
fn test_new() {
    let id = ServiceID::SMDB;
    let name = String::from("name");
    let port = 80;
    let protocol = ProtocolType::HTTP;

    let config = MainConfig::new(id, name, port, protocol);

    assert_eq!(config.id(), &ServiceID::SMDB);
    assert_eq!(config.name(), &String::from("name"));
    assert_eq!(config.port(), 80);
    assert_eq!(config.protocol(), &ProtocolType::HTTP);
}

#[test]
fn test_default() {
    let config = MainConfig::default();

    assert_eq!(config.id(), &ServiceID::SMDB);
    assert_eq!(config.name(), &String::from(""));
    assert_eq!(config.port(), 0);
    assert_eq!(config.protocol(), &ProtocolType::GRPC);
}

#[test]
fn test_debug() {
    let config = MainConfig::default();

    let expected = "MainConfig { id: SMDB, name: \"\", port: 0, protocol: GRPC }";
    let actual = format!("{:?}", config);
    assert_eq!(expected, actual);
}

#[test]
fn test_display() {
    let config = MainConfig::default();

    let expected = "MainConfig { id: SMDB, name: \"\", port: 0, protocol: GRPC }";
    let actual = config.to_string();
    assert_eq!(expected, actual);
}
