use common_exchange::ExchangeID;
use common_order::OrderCancelAll;

#[test]
fn test_order_cancel_all_creation() {
    let cancel_all = OrderCancelAll::new(ExchangeID::Binance);
    assert_eq!(cancel_all.exchange_id(), ExchangeID::Binance);
}

#[test]
fn test_order_cancel_all_default() {
    let default_cancel = OrderCancelAll::default();
    assert_eq!(default_cancel.exchange_id(), ExchangeID::default());
}

#[test]
fn test_order_cancel_all_into_exchange_id() {
    let cancel_all = OrderCancelAll::new(ExchangeID::Binance);
    let exchange_id: ExchangeID = cancel_all.into();
    assert_eq!(exchange_id, ExchangeID::Binance);
}

#[test]
fn test_order_cancel_all_display() {
    let cancel_all = OrderCancelAll::new(ExchangeID::Binance);
    let display_string = cancel_all.to_string();
    assert!(display_string.contains("Binance"));
}

#[test]
fn test_order_cancel_all_debug() {
    let cancel_all = OrderCancelAll::new(ExchangeID::Binance);
    let debug_string = format!("{:?}", cancel_all);
    assert!(debug_string.contains("Binance"));
}

#[test]
fn test_order_cancel_all_clone_and_eq() {
    let original = OrderCancelAll::new(ExchangeID::Binance);
    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderCancelAll::new(ExchangeID::default());
    assert_ne!(original, different);
}
