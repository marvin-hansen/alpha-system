use common_exchange::prelude::ExchangeID;
use common_ims::prelude::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};

#[test]
fn test_integration_config_initialization() {
    let integration_id = "test-integration".to_string();
    let ims_type = ImsIntegrationType::Data;
    let exchange_id = ExchangeID::Binance;
    let msg_config = IntegrationMessageConfig::new(1, exchange_id);

    let config = IntegrationConfig::new(
        integration_id.clone(),
        ims_type,
        exchange_id,
        msg_config.clone(),
    );

    assert_eq!(config.integration_id(), integration_id);
    assert_eq!(config.ims_integration_type(), ims_type);
    assert_eq!(config.exchange_id(), exchange_id);
    assert_eq!(config.integration_message_config(), &msg_config);
}

#[test]
fn test_integration_config_display() {
    let integration_id = "test-display".to_string();
    let config = IntegrationConfig::new(
        integration_id.clone(),
        ImsIntegrationType::OMS,
        ExchangeID::Kraken,
        IntegrationMessageConfig::new(1, ExchangeID::Kraken),
    );

    assert_eq!(config.to_string(), integration_id);
}

#[test]
fn test_integration_config_getters() {
    let integration_id = "test-getters".to_string();
    let ims_type = ImsIntegrationType::Execution;
    let exchange_id = ExchangeID::Binance;
    let msg_config = IntegrationMessageConfig::new(2, exchange_id);

    let config = IntegrationConfig::new(
        integration_id.clone(),
        ims_type,
        exchange_id,
        msg_config.clone(),
    );

    assert_eq!(config.integration_id(), &integration_id);
    assert_eq!(config.ims_integration_type(), ims_type);
    assert_eq!(config.exchange_id(), exchange_id);
    assert_eq!(config.integration_message_config(), &msg_config);
}
