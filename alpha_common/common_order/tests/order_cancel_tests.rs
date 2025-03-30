/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_exchange::ExchangeID;
use common_order::OrderCancel;

#[test]
fn test_order_cancel_single_display() {
    let cancel_order = OrderCancel::new(
        ExchangeID::BinanceSpot,
        1,
        "cl_ord_id".into(),
        "exchange_order_id".into(),
    );

    let display_string = cancel_order.to_string();
    assert!(display_string.contains("Binance"));
    assert!(display_string.contains("cl_ord_id"));
    assert!(display_string.contains("exchange_order_id"));
}

#[test]
fn test_order_cancel_single_debug() {
    let cancel_order = OrderCancel::new(
        ExchangeID::BinanceSpot,
        1,
        "cl_ord_id".into(),
        "exchange_order_id".into(),
    );

    let debug_string = format!("{:?}", cancel_order);
    assert!(debug_string.contains("Binance"));
    assert!(debug_string.contains("cl_ord_id"));
    assert!(debug_string.contains("exchange_order_id"));
}

#[test]
fn test_order_cancel_single_clone_and_eq() {
    let original = OrderCancel::new(
        ExchangeID::BinanceSpot,
        1,
        "cl_ord_id".into(),
        "exchange_order_id".into(),
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderCancel::new(
        ExchangeID::BinanceSpot,
        42,
        "cl_ord_id".into(),
        "exchange_order_id".into(),
    );
    assert_ne!(original, different);
}

#[test]
fn test_order_cancel_single_with_different_exchanges() {
    let binance_cancel = OrderCancel::new(
        ExchangeID::BinanceSpot,
        1,
        "cl_ord_id".into(),
        "binance_order_id".into(),
    );

    let kraken_cancel = OrderCancel::new(
        ExchangeID::Kraken,
        1,
        "dif_ord_id".into(),
        "kraken_order_id".into(),
    );
    assert_eq!(binance_cancel.exchange_id(), ExchangeID::BinanceSpot);
    assert_ne!(binance_cancel, kraken_cancel);

    assert_eq!(kraken_cancel.exchange_id(), ExchangeID::Kraken);
}
