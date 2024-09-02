use common_config::prelude::{Endpoint, ProtocolType};
use proto_smdb::proto::ProtoEndpoint;
use proto_smdb_utils::endpoint_proto_utils::{endpoint_from_proto, endpoint_to_proto};

#[test]
fn test_from_proto() {
    let proto = ProtoEndpoint {
        name: "test".to_string(),
        version: 1,
        uri: "http://localhost".to_string(),
        port: 8080,
        protocol: 1,
    };

    let all_endpoints = endpoint_from_proto(Vec::from([proto])).unwrap();
    let endpoint = all_endpoints.get(0).unwrap();

    assert_eq!(endpoint.name(), "test");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "http://localhost");
    assert_eq!(endpoint.port(), 8080);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
}

#[test]
fn test_to_proto() {
    let endpoint = Endpoint::new(
        "test".to_string(),
        1,
        "http://localhost".to_string(),
        8080,
        ProtocolType::GRPC,
    );

    let all_endpoints = endpoint_to_proto(&Vec::from([endpoint])).unwrap();
    let proto = all_endpoints.get(0).unwrap();

    assert_eq!(proto.name, "test");
    assert_eq!(proto.version, 1);
    assert_eq!(proto.uri, "http://localhost");
    assert_eq!(proto.port, 8080);
    assert_eq!(proto.protocol, 1);
}
