use common_exchange::ExchangeID;
use common_order::{OrderCreate, OrderSide, OrderType, TimeInForce};
use rust_decimal::Decimal;

#[test]
fn test_order_new_single_with_expiry() {
    let expiry_time = 1639123200;
    let order = OrderCreate::new(
        ExchangeID::BinanceSpot,
        1, // client_order_id
        "cl_ord_id".into(),
        "ETHUSD".into(),
        OrderType::Market,
        OrderSide::Sell,
        TimeInForce::GoodTillTimeExchange,
        Some(expiry_time),
        Decimal::new(20, 1),
        Decimal::new(300000000, 1),
    );

    assert_eq!(
        order.order_time_in_force(),
        &TimeInForce::GoodTillTimeExchange
    );
    assert_eq!(order.time_expiry(), Some(expiry_time));
}

#[test]
fn test_order_new_single_display() {
    let order = OrderCreate::new(
        ExchangeID::BinanceSpot,
        1, // client_order_id
        "cl_ord_id".into(),
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    let display_string = order.to_string();
    println!("Display string: {}", display_string);

    let debug_string = format!("{:?}", order);
    println!("Debug string: {}", debug_string);

    // Verify the display string contains all the important information
    assert!(display_string.contains("cl_ord_id"));
    assert!(display_string.contains("BTCUSD"));
}
#[test]
fn test_order_new_single_clone_and_eq() {
    let original = OrderCreate::new(
        ExchangeID::BinanceSpot,
        1, // client_order_id
        "ord_id".into(),
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different_order = OrderCreate::new(
        ExchangeID::default(),
        42,               // client_id
        "diff_id".into(), // Different order ID
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    assert_ne!(original, different_order);
}

#[test]
fn test_order_new_single_with_different_exchanges() {
    let binance_order = OrderCreate::new(
        ExchangeID::BinanceSpot,
        1,
        "cl_ord_id".into(), // client_order_id
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    let kraken_order = OrderCreate::new(
        ExchangeID::Kraken,
        1, // client_order_id
        "cl_ord_id".into(),
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    assert_ne!(binance_order, kraken_order);
    assert_eq!(binance_order.exchange_id(), ExchangeID::BinanceSpot);
    assert_eq!(kraken_order.exchange_id(), ExchangeID::Kraken);
}
