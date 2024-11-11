use common_ims::prelude::ExchangeID;
use common_ims::prelude::IntegrationMessageConfig;

#[test]
fn test_new_config_name_format() {
    let exchange_id = ExchangeID::Binance;
    let id = 123;
    let version = 1;
    let config = IntegrationMessageConfig::new(id, version, exchange_id);

    assert_eq!(config.name(), "Binance-integration-123");
}

#[test]
fn test_channel_name_generation() {
    let version = 1;

    let config = IntegrationMessageConfig::new(1, version, ExchangeID::Binance);

    assert_eq!(config.control_channel(), "Binance-integration-1-control");
    assert_eq!(config.data_channel(), "Binance-integration-1-data");
    assert_eq!(config.error_channel(), "Binance-integration-1-error");
    assert_eq!(
        config.execution_channel(),
        "Binance-integration-1-execution"
    );
    assert_eq!(
        config.heartbeat_channel(),
        "Binance-integration-1-heartbeat"
    );
}

#[test]
fn test_getter_methods_return_original_values() {
    let id = 456;
    let exchange_id = ExchangeID::Kraken;
    let version = 1;

    let config = IntegrationMessageConfig::new(id, version, exchange_id);

    assert_eq!(config.id(), id);
    assert_eq!(config.exchange_id(), exchange_id);
    assert_eq!(config.version(), &version);
    assert_eq!(config.name(), "Kraken-integration-456");
}
