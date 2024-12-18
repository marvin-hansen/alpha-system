use common_exchange::ExchangeID;
use common_order::{
    ClientOrderID, ExchangeOrderID, OrderSide, OrderType, OrderUpdate, TimeInForce,
};
use rust_decimal::Decimal;

#[test]
fn test_order_update_new() {
    let exchange_id = ExchangeID::Binance;
    let client_id = 1;
    let client_order_id = ClientOrderID::new("cl_ord_id");
    let exchange_order_id = ExchangeOrderID::new("exchange_order_id".to_string());
    let symbol_id = "BTC-USD".to_string();
    let order_type = OrderType::Limit;
    let order_side = OrderSide::Buy;
    let time_in_force = TimeInForce::GoodTillCancel;
    let time_expiry = None;
    let price = Decimal::new(105, 1);
    let quantity = Decimal::new(12, 1);

    let order_update = OrderUpdate::new(
        exchange_id,
        client_id,
        client_order_id,
        exchange_order_id,
        symbol_id,
        order_type,
        order_side,
        time_in_force,
        time_expiry,
        price,
        quantity,
    );

    assert_eq!(order_update.exchange_id(), exchange_id);
    assert_eq!(order_update.client_id(), client_id);
    assert_eq!(order_update.client_order_id(), "cl_ord_id");
    assert_eq!(order_update.exchange_order_id(), "exchange_order_id");
    assert_eq!(order_update.symbol_id(), "BTC-USD");
    assert_eq!(order_update.order_type(), &OrderType::Limit);
    assert_eq!(order_update.order_side(), &OrderSide::Buy);
    assert_eq!(order_update.time_in_force(), &TimeInForce::GoodTillCancel);
    assert_eq!(order_update.time_expiry(), None);
    assert_eq!(order_update.price(), Decimal::new(105, 1));
    assert_eq!(order_update.quantity(), Decimal::new(12, 1));
}

#[test]
fn test_order_update_clone_and_eq() {
    let original = OrderUpdate::new(
        ExchangeID::Binance,
        1,
        ClientOrderID::new("cl_ord_id"),
        ExchangeOrderID::new("exchange_order_id".to_string()),
        "BTC-USD".to_string(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(105, 1),
        Decimal::new(12, 1),
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);

    let different = OrderUpdate::new(
        ExchangeID::Kraken,
        2,
        ClientOrderID::new("diff_id"),
        ExchangeOrderID::new("diff_exc_order_id".to_string()),
        "ETH-EUR".to_string(),
        OrderType::Market,
        OrderSide::Sell,
        TimeInForce::FillOrKill,
        Some(1643723900),
        Decimal::new(208, 1),
        Decimal::new(34, 1),
    );
    assert_ne!(original, different);
}

#[test]
fn test_order_update_display() {
    let order_update = OrderUpdate::new(
        ExchangeID::Binance,
        1,
        ClientOrderID::new("cl_ord_id"),
        ExchangeOrderID::new("exchange_order_id".to_string()),
        "BTC-USD".to_string(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(105, 1),
        Decimal::new(12, 1),
    );

    let display_string = order_update.to_string();
    assert!(display_string.contains("cl_ord_id"));
    assert!(display_string.contains("exchange_order_id"));
    assert!(display_string.contains("BTC-USD"));
}
