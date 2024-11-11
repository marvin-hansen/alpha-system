use proto_imdb::proto::IntegrationConfig;

#[test]
fn test_proto_integration_config() {
    let integration_config = IntegrationConfig {
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
    assert_eq!(integration_config.online, true);
    assert_eq!(integration_config.exchange_id, 1);
    assert_eq!(integration_config.integration_message_config, None);
}
