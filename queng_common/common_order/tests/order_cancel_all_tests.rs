use common_exchange::ExchangeID;
use common_order::OrderCancelAll;

#[test]
fn test_order_cancel_all_creation() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let cancel_all = OrderCancelAll::new(exchange_id, client_id);
    assert_eq!(cancel_all.exchange_id(), ExchangeID::BinanceSpot);
    assert_eq!(cancel_all.client_id(), 1);
}

#[test]
fn test_order_cancel_all_default() {
    let default_cancel = OrderCancelAll::default();
    assert_eq!(default_cancel.exchange_id(), ExchangeID::default());
}

#[test]
fn test_order_cancel_all_display() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let cancel_all = OrderCancelAll::new(exchange_id, client_id);
    let display_string = cancel_all.to_string();
    assert!(display_string.contains("Binance"));
}

#[test]
fn test_order_cancel_all_debug() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let cancel_all = OrderCancelAll::new(exchange_id, client_id);

    let debug_string = format!("{:?}", cancel_all);
    assert!(debug_string.contains("Binance"));
}

#[test]
fn test_order_cancel_all_clone_and_eq() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;

    let original = OrderCancelAll::new(exchange_id, client_id);
    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderCancelAll::new(ExchangeID::Kraken, 43);
    assert_ne!(original, different);
}
