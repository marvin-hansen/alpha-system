/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use proto_imdb::proto::ProtoIntegrationConfig;

#[test]
fn test_proto_integration_config() {
    let integration_config = ProtoIntegrationConfig {
        integration_id: "test".to_string(),
        integration_version: 1,
        ims_integration_type: 1,
        online: true,
        exchange_id: 1,
        integration_message_config: None,
    };

    assert_eq!(integration_config.integration_id, "test");
    assert_eq!(integration_config.integration_version, 1);
    assert_eq!(integration_config.ims_integration_type, 1);
    assert!(integration_config.online);
    assert_eq!(integration_config.exchange_id, 1);
    assert_eq!(integration_config.integration_message_config, None);
}
