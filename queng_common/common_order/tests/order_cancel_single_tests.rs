use common_exchange::ExchangeID;
use common_order::OrderCancelSingle;

#[test]
fn test_order_cancel_single_creation() {
    let cancel_order = OrderCancelSingle::new(ExchangeID::Binance, "BTC-USD".to_string());

    assert_eq!(cancel_order.exchange_id(), ExchangeID::Binance);
    assert_eq!(cancel_order.symbol_id(), "BTC-USD");
}

#[test]
fn test_order_cancel_single_default() {
    let default_cancel = OrderCancelSingle::default();
    assert_eq!(default_cancel.exchange_id(), ExchangeID::default());
    assert_eq!(default_cancel.symbol_id(), "");
}

#[test]
fn test_order_cancel_single_display() {
    let cancel_order = OrderCancelSingle::new(ExchangeID::Binance, "BTC-USD".to_string());

    let display_string = cancel_order.to_string();
    assert!(display_string.contains("BTC-USD"));
    assert!(display_string.contains("Binance"));
}

#[test]
fn test_order_cancel_single_debug() {
    let cancel_order = OrderCancelSingle::new(ExchangeID::Binance, "BTC-USD".to_string());

    let debug_string = format!("{:?}", cancel_order);
    assert!(debug_string.contains("BTC-USD"));
    assert!(debug_string.contains("Binance"));
}

#[test]
fn test_order_cancel_single_clone_and_eq() {
    let original = OrderCancelSingle::new(ExchangeID::Binance, "BTC-USD".to_string());

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderCancelSingle::new(ExchangeID::default(), "ETH-USD".to_string());

    assert_ne!(original, different);
}

#[test]
fn test_order_cancel_single_with_different_exchanges() {
    let binance_cancel = OrderCancelSingle::new(ExchangeID::Binance, "BTC-USD".to_string());

    let kraken_cancel = OrderCancelSingle::new(ExchangeID::Kraken, "BTC-USD".to_string());

    assert_ne!(binance_cancel, kraken_cancel);
    assert_eq!(binance_cancel.exchange_id(), ExchangeID::Binance);
    assert_eq!(kraken_cancel.exchange_id(), ExchangeID::Kraken);
    assert_eq!(binance_cancel.symbol_id(), kraken_cancel.symbol_id());
}
