/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use proto_imdb::proto::ProtoIntegrationConfig;
use proto_imdb_utils::*;

#[test]
fn test_integration_config_roundtrip() {
    // Create test IntegrationConfig
    let message_config = IntegrationMessageConfig::new(1, 2, ExchangeID::from(123));
    let config = IntegrationConfig::new(
        "test_integration".to_string(),
        1,
        ImsIntegrationType::from(1_u16),
        ExchangeID::from(123),
        message_config,
    );

    // Convert to proto and back
    let proto = integration_config_to_proto(config.clone());
    let converted = integration_config_from_proto(proto);

    // Verify fields match
    assert_eq!(converted.integration_id(), config.integration_id());
    assert_eq!(
        converted.integration_version(),
        config.integration_version()
    );
    assert_eq!(
        converted.ims_integration_type(),
        config.ims_integration_type()
    );
    assert_eq!(converted.exchange_id(), config.exchange_id());

    let orig_msg = config.integration_message_config();
    let conv_msg = converted.integration_message_config();
    assert_eq!(conv_msg.id(), orig_msg.id());
    assert_eq!(conv_msg.version(), orig_msg.version());
    assert_eq!(conv_msg.exchange_id(), orig_msg.exchange_id());
}

#[test]
fn test_default_message_config() {
    let proto = ProtoIntegrationConfig {
        integration_id: "test".to_string(),
        integration_version: 1,
        ims_integration_type: 1,
        online: true,
        exchange_id: 123,
        integration_message_config: None,
    };

    let config = integration_config_from_proto(proto);
    let msg_config = config.integration_message_config();

    assert_eq!(msg_config.id(), 1);
    assert_eq!(*msg_config.version(), 1);
    assert_eq!(msg_config.exchange_id(), ExchangeID::from(123));
}
