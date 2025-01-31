/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use proto_smdb::proto::{ProtoEndpoint, ProtoServiceConfig};

#[test]
fn test_proto_service_config() {
    let proto = ProtoServiceConfig {
        svc_id: 1,
        name: "Test Service".to_string(),
        version: 1,
        online: true,
        description: "Test description".to_string(),
        health_check_uri: "/health".to_string(),
        base_uri: "http://localhost:8080".to_string(),
        dependencies: vec![2, 3],
        endpoint: Vec::from([
            ProtoEndpoint::default(),
            ProtoEndpoint::default(),
            ProtoEndpoint::default(),
        ]),
    };

    assert_eq!(proto.svc_id, 1);
    assert_eq!(proto.name, "Test Service");
    assert_eq!(proto.version, 1);
    assert!(proto.online);
    assert_eq!(proto.description, "Test description");
    assert_eq!(proto.health_check_uri, "/health");
    assert_eq!(proto.base_uri, "http://localhost:8080");
    assert_eq!(proto.dependencies, vec![2, 3]);
    assert_eq!(proto.endpoint.len(), 3);
}
