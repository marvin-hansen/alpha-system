use common_exchange::ExchangeID;
use common_order::{OrderSide, OrderSingleNew, OrderType, TimeInForce};
use rust_decimal::Decimal;

#[test]
fn test_order_new_single_with_expiry() {
    let expiry_time = 1639123200;
    let order = OrderSingleNew::new(
        ExchangeID::Binance,
        001, // client_order_id
        "test_order_2".to_string().into(),
        "ETH-USD".to_string(),
        "ETHUSD".to_string(),
        OrderType::Market,
        OrderSide::Sell,
        TimeInForce::GoodTillTimeExchange,
        Some(expiry_time),
        Decimal::new(20, 1),
        Decimal::new(300000000, 1),
    );

    assert_eq!(order.time_in_force(), &TimeInForce::GoodTillTimeExchange);
    assert_eq!(order.time_expiry(), Some(expiry_time));
}

#[test]
fn test_order_new_single_display() {
    let order = OrderSingleNew::new(
        ExchangeID::Binance,
        001, // client_order_id
        "test_order_3".to_string().into(),
        "BTC-USD".to_string(),
        "BTCUSD".to_string(),
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
    assert!(display_string.contains("test_order_3"));
    assert!(display_string.contains("BTC-USD"));
    assert!(display_string.contains("BTCUSD"));
}
#[test]
fn test_order_new_single_clone_and_eq() {
    let original = OrderSingleNew::new(
        ExchangeID::Binance,
        001, // client_order_id
        "test_order_5".to_string().into(),
        "BTC-USD".to_string(),
        "BTCUSD".to_string(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different_order = OrderSingleNew::new(
        ExchangeID::default(),
        042,                               // client_order_id
        "test_order_6".to_string().into(), // Different order ID
        "BTC-USD".to_string(),
        "BTCUSD".to_string(),
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
    let binance_order = OrderSingleNew::new(
        ExchangeID::Binance,
        001, // client_order_id
        "binance_order".to_string().into(),
        "BTC-USD".to_string(),
        "BTCUSD".to_string(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    let kraken_order = OrderSingleNew::new(
        ExchangeID::Kraken,
        001, // client_order_id
        "kraken_order".to_string().into(),
        "BTC-USD".to_string(),
        "BTCUSD".to_string(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    assert_ne!(binance_order, kraken_order);
    assert_eq!(binance_order.exchange_id(), ExchangeID::Binance);
    assert_eq!(kraken_order.exchange_id(), ExchangeID::Kraken);
}
