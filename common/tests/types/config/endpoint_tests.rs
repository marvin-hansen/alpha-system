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

    assert_eq!(endpoint.endpoint_name(), &String::from("name"));
    assert_eq!(endpoint.endpoint_version(), 1);
    assert_eq!(
        endpoint.endpoint_description(),
        &String::from("description")
    );
    assert_eq!(endpoint.endpoint_uri(), &String::from("/"));
    assert_eq!(endpoint.endpoint_port(), 8080);
    assert_eq!(endpoint.endpoint_protocol(), &Protocol::GRPC);
    assert_eq!(endpoint.endpoint_encoding(), &Encoding::Protobuf);
}

#[test]
fn test_default() {
    let endpoint = Endpoint::default();

    assert_eq!(endpoint.endpoint_name(), &String::from(""));
    assert_eq!(endpoint.endpoint_version(), 0);
    assert_eq!(endpoint.endpoint_description(), &String::from(""));
    assert_eq!(endpoint.endpoint_uri(), &String::from(""));
    assert_eq!(endpoint.endpoint_port(), 0);
    assert_eq!(endpoint.endpoint_protocol(), &Protocol::GRPC);
    assert_eq!(endpoint.endpoint_encoding(), &Encoding::Protobuf);
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
               "endpoint_name: name, endpoint_version: 1, endpoint_port: 8080, endpoint_description: description, endpoint_uri: /, endpoint_protocol: GRPC, endpoint_encoding: Protobuf");
}
