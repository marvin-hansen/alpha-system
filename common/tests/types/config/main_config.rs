use common::types::config::main_config::MainConfig;
use common::types::config::protocol::Protocol;

#[test]
fn test_new() {
    let id = String::from("id");
    let name = String::from("name");
    let port = 80;
    let protocol = Protocol::HTTP;

    let config = MainConfig::new(id, name, port, protocol);

    assert_eq!(config.id(), &String::from("id"));
    assert_eq!(config.name(), &String::from("name"));
    assert_eq!(config.port(), 80);
    assert_eq!(config.protocol(), &Protocol::HTTP);
}

#[test]
fn test_default() {
    let config = MainConfig::default();

    assert_eq!(config.id(), &String::from("id"));
    assert_eq!(config.name(), &String::from("name"));
    assert_eq!(config.port(), 80);
    assert_eq!(config.protocol(), &Protocol::HTTP);
}

#[test]
fn test_debug() {
    let config = MainConfig::default();

    let expected = "MainConfig { id: \"id\", name: \"name\", port: 80, protocol: HTTP }";
    let actual = format!("{:?}", config);
    assert_eq!(expected, actual);
}

#[test]
fn test_display() {
    let config = MainConfig::default();

    let expected = "MainConfig { id: \"id\", name: \"name\", port: 80, protocol: HTTP }";
    let actual = config.to_string();
    assert_eq!(expected, actual);
}