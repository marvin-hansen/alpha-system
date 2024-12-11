use common_exchange::ExchangeID;
use common_order::OrderCancelSingle;

#[test]
fn test_order_cancel_single_creation() {
    let cancel_order = OrderCancelSingle::new(
        ExchangeID::Binance,
        "BTC-USD".to_string(),
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    assert_eq!(cancel_order.exchange_id(), ExchangeID::Binance);
    assert_eq!(cancel_order.symbol_id(), "BTC-USD");
    assert_eq!(cancel_order.client_order_id(), "client_order_id");
    assert_eq!(cancel_order.exchange_order_id(), "exchange_order_id");
}

#[test]
fn test_order_cancel_single_default() {
    let default_cancel = OrderCancelSingle::default();
    assert_eq!(default_cancel.exchange_id(), ExchangeID::default());
    assert_eq!(default_cancel.symbol_id(), "");
    assert_eq!(default_cancel.client_order_id(), "");
    assert_eq!(default_cancel.exchange_order_id(), "");
}

#[test]
fn test_order_cancel_single_display() {
    let cancel_order = OrderCancelSingle::new(
        ExchangeID::Binance,
        "BTC-USD".to_string(),
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let display_string = cancel_order.to_string();
    assert!(display_string.contains("BTC-USD"));
    assert!(display_string.contains("Binance"));
    assert!(display_string.contains("client_order_id"));
    assert!(display_string.contains("exchange_order_id"));
}

#[test]
fn test_order_cancel_single_debug() {
    let cancel_order = OrderCancelSingle::new(
        ExchangeID::Binance,
        "BTC-USD".to_string(),
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let debug_string = format!("{:?}", cancel_order);
    assert!(debug_string.contains("BTC-USD"));
    assert!(debug_string.contains("Binance"));
    assert!(debug_string.contains("client_order_id"));
    assert!(debug_string.contains("exchange_order_id"));
}

#[test]
fn test_order_cancel_single_clone_and_eq() {
    let original = OrderCancelSingle::new(
        ExchangeID::Binance,
        "BTC-USD".to_string(),
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderCancelSingle::new(
        ExchangeID::Binance,
        "ETH-EUR".to_string(),
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );
    assert_ne!(original, different);
}

#[test]
fn test_order_cancel_single_with_different_exchanges() {
    let binance_cancel = OrderCancelSingle::new(
        ExchangeID::Binance,
        "BTC-USD".to_string(),
        "binance_client_order_id".to_string(),
        "binance_exchange_order_id".to_string(),
    );

    let kraken_cancel = OrderCancelSingle::new(
        ExchangeID::Kraken,
        "BTC-USD".to_string(),
        "kraken_client_order_id".to_string(),
        "kraken_exchange_order_id".to_string(),
    );
    assert_eq!(binance_cancel.exchange_id(), ExchangeID::Binance);
    assert_ne!(binance_cancel, kraken_cancel);

    assert_eq!(kraken_cancel.exchange_id(), ExchangeID::Kraken);
    assert_eq!(binance_cancel.symbol_id(), kraken_cancel.symbol_id());
}
