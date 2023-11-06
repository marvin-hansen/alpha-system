use common::types::config::encoding::Encoding;
use common::types::config::endpoint::Endpoint;
use common::types::config::protocol::Protocol;

#[test]
fn test_new() {
    let name = String::from("name");
    let version = 1;
    let description = String::from("description");
    let uri = String::from("/");
    let port = 8080;
    let protocol = Protocol::GRPC;
    let encoding = Encoding::Protobuf;

    let endpoint = Endpoint::new(name, version, description, uri, port, protocol, encoding);

    assert_eq!(endpoint.name(), &String::from("name"));
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.description(), &String::from("description"));
    assert_eq!(endpoint.uri(), &String::from("/"));
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), &Protocol::GRPC);
    assert_eq!(endpoint.encoding(), &Encoding::Protobuf);
}

#[test]
fn test_default() {
    let endpoint = Endpoint::default();

    assert_eq!(endpoint.name(), &String::from(""));
    assert_eq!(endpoint.version(), 0);
    assert_eq!(endpoint.description(), &String::from(""));
    assert_eq!(endpoint.uri(), &String::from(""));
    assert_eq!(endpoint.port(), 0);
    assert_eq!(endpoint.protocol(), &Protocol::GRPC);
    assert_eq!(endpoint.encoding(), &Encoding::Protobuf);
}

#[test]
fn test_display() {
    let name = String::from("name");
    let version = 1;
    let description = String::from("description");
    let uri = String::from("/");
    let port = 8080;
    let protocol = Protocol::GRPC;
    let encoding = Encoding::Protobuf;

    let endpoint = Endpoint::new(name, version, description, uri, port, protocol, encoding);

    assert_eq!(endpoint.to_string(),
               "name: name,  version: 1,  port: 8080,  description: description,  uri: /,  protocol: GRPC,  encoding: Protobuf");
}
