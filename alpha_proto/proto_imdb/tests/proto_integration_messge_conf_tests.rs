/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use proto_imdb::proto::ProtoIntegrationMessageConfig;

#[test]
fn test_proto_integration_message_config() {
    let integration_message_config = ProtoIntegrationMessageConfig {
        id: 1,
        name: "test".to_string(),
        version: 1,
        exchange_id: 1,
    };

    assert_eq!(integration_message_config.id, 1);
    assert_eq!(integration_message_config.name, "test");
    assert_eq!(integration_message_config.version, 1);
    assert_eq!(integration_message_config.exchange_id, 1);
}
