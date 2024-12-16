use common_exchange::ExchangeID;
use common_order::OrderCancel;

#[test]
fn test_order_cancel_single_default() {
    let default_cancel = OrderCancel::default();
    assert_eq!(default_cancel.exchange_id(), ExchangeID::default());
    assert_eq!(default_cancel.client_order_id(), "");
    assert_eq!(default_cancel.exchange_order_id(), "");
}

#[test]
fn test_order_cancel_single_display() {
    let cancel_order = OrderCancel::new(
        ExchangeID::Binance,
        1,
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let display_string = cancel_order.to_string();
    assert!(display_string.contains("Binance"));
    assert!(display_string.contains("client_order_id"));
    assert!(display_string.contains("exchange_order_id"));
}

#[test]
fn test_order_cancel_single_debug() {
    let cancel_order = OrderCancel::new(
        ExchangeID::Binance,
        1,
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let debug_string = format!("{:?}", cancel_order);
    assert!(debug_string.contains("Binance"));
    assert!(debug_string.contains("client_order_id"));
    assert!(debug_string.contains("exchange_order_id"));
}

#[test]
fn test_order_cancel_single_clone_and_eq() {
    let original = OrderCancel::new(
        ExchangeID::Binance,
        1,
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderCancel::new(
        ExchangeID::Binance,
        42,
        "client_order_id".to_string(),
        "exchange_order_id".to_string(),
    );
    assert_ne!(original, different);
}

#[test]
fn test_order_cancel_single_with_different_exchanges() {
    let binance_cancel = OrderCancel::new(
        ExchangeID::Binance,
        1,
        "binance_client_order_id".to_string(),
        "binance_exchange_order_id".to_string(),
    );

    let kraken_cancel = OrderCancel::new(
        ExchangeID::Kraken,
        1,
        "kraken_client_order_id".to_string(),
        "kraken_exchange_order_id".to_string(),
    );
    assert_eq!(binance_cancel.exchange_id(), ExchangeID::Binance);
    assert_ne!(binance_cancel, kraken_cancel);

    assert_eq!(kraken_cancel.exchange_id(), ExchangeID::Kraken);
}
