use common::prelude::{Encoding, Endpoint, ProtocolType};
use proto::binding::ProtoEndpoint;

#[test]
fn test_new() {
    let name = "name".to_string();
    let version = 1;
    let description = "description".to_string();
    let uri = "/".to_string();
    let port = 8080;
    let protocol = ProtocolType::GRPC;
    let encoding = Encoding::Protobuf;

    let endpoint = Endpoint::new(name, version, description, uri, port, protocol, encoding);

    assert_eq!(endpoint.name(), "name");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.description(), "description");
    assert_eq!(endpoint.uri(), &String::from("/"));
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}

#[test]
fn test_default() {
    let endpoint = Endpoint::default();

    assert_eq!(endpoint.name(), "");
    assert_eq!(endpoint.version(), 0);
    assert_eq!(endpoint.description(), "");
    assert_eq!(endpoint.uri(), "");
    assert_eq!(endpoint.port(), 0);
    assert_eq!(endpoint.protocol(), ProtocolType::NullVal);
    assert_eq!(endpoint.encoding(), Encoding::NullVal);
}

#[test]
fn test_from_proto() {
    let proto = ProtoEndpoint {
        name: "test".to_string(),
        version: 1,
        description: "description".to_string(),
        uri: "http://localhost".to_string(),
        port: 8080,
        protocol: 1,
        encoding: 2,
    };

    let endpoint = Endpoint::from_proto(proto).unwrap();

    assert_eq!(endpoint.name(), "test");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.description(), "description");
    assert_eq!(endpoint.uri(), "http://localhost");
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}

#[test]
fn test_to_proto() {
    let endpoint = Endpoint::new(
        "test".to_string(),
        1,
        "description".to_string(),
        "http://localhost".to_string(),
        8080,
        ProtocolType::GRPC,
        Encoding::Protobuf,
    );

    let proto = endpoint.to_proto().unwrap();

    assert_eq!(proto.name, "test");
    assert_eq!(proto.version, 1);
    assert_eq!(proto.description, "description");
    assert_eq!(proto.uri, "http://localhost");
    assert_eq!(proto.port, 8080);
    assert_eq!(proto.protocol, 1);
    assert_eq!(proto.encoding, 2);
}

#[test]
fn test_host_endpoint() {
    let endpoint = Endpoint::default();

    let host_endpoint = endpoint.host_endpoint();

    assert_eq!(host_endpoint.host_uri(), "");
    assert_eq!(host_endpoint.port(), 0);
}

#[test]
fn test_display() {
    let name = "name".to_string();
    let version = 1;
    let description = "description".to_string();
    let uri = "/".to_string();
    let port = 8080;
    let protocol = ProtocolType::GRPC;
    let encoding = Encoding::Protobuf;

    let endpoint = Endpoint::new(name, version, description, uri, port, protocol, encoding);

    assert_eq!(endpoint.to_string(),
               "name: name,  version: 1,  port: 8080,  description: description,  uri: /,  protocol: GRPC,  encoding: Protobuf");
}
