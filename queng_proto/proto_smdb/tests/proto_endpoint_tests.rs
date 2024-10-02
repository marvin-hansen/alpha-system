use proto_smdb::proto::ProtoEndpoint;

#[test]
fn test_proto_endpoint() {
    let proto = ProtoEndpoint {
        name: "test".to_string(),
        version: 1,
        uri: "http://localhost".to_string(),
        port: 8080,
        protocol: 1,
    };

    assert_eq!(proto.name, "test");
    assert_eq!(proto.version, 1);
    assert_eq!(proto.uri, "http://localhost");
    assert_eq!(proto.port, 8080);
    assert_eq!(proto.protocol, 1);
}
