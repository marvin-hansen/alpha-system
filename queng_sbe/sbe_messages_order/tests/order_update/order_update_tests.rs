use common_exchange::ExchangeID;
use common_order::{
    ClientOrderID, ExchangeOrderID, OrderExchangeSymbol, OrderSide, OrderType, OrderUpdate,
    TimeInForce,
};
use rust_decimal::Decimal;
use sbe_messages_order::{decode_order_update_message, encode_order_update_message};

#[test]
fn test_encode_order_update_message() {
    let order_update = OrderUpdate::new(
        ExchangeID::BinanceSpot,
        1,
        "cl_ord_id".into(),
        "exchange_order_id".into(),
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(105, 1),
        Decimal::new(12, 1),
    );

    let result = encode_order_update_message(order_update);
    assert!(result.is_ok());

    let (size, buffer) = result.unwrap();
    assert_eq!(size, 91); // Assert encoded message size matches expected
    assert!(!buffer.is_empty()); // Assert non-empty encoded message

    let expected: Vec<u8> = vec![
        83, 0, 146, 1, 1, 0, 1, 0, 146, 1, 4, 1, 0, 228, 247, 147, 162, 247, 151, 39, 0, 91, 124,
        142, 29, 200, 110, 191, 7, 34, 185, 137, 255, 71, 2, 0, 0, 148, 208, 48, 135, 2, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 1, 105,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode_order_update_message() {
    let encoded: Vec<u8> = vec![
        83, 0, 146, 1, 1, 0, 1, 0, 146, 1, 4, 1, 0, 228, 247, 147, 162, 247, 151, 39, 0, 91, 124,
        142, 29, 200, 110, 191, 7, 34, 185, 137, 255, 71, 2, 0, 0, 148, 208, 48, 135, 2, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 1, 105,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let result = decode_order_update_message(encoded.as_slice());
    assert!(result.is_ok());

    let order_update = result.unwrap();
    assert_eq!(order_update.exchange_id(), ExchangeID::BinanceSpot);
    assert_eq!(order_update.client_id(), 1);
    assert_eq!(
        order_update.client_order_id(),
        &ClientOrderID::from("cl_ord_id")
    );
    assert_eq!(
        order_update.exchange_order_id(),
        &ExchangeOrderID::from("exchange_order_id")
    );
    assert_eq!(
        order_update.symbol_id_exchange(),
        &OrderExchangeSymbol::from("BTCUSD")
    );
    assert_eq!(order_update.order_type(), &OrderType::Limit);
    assert_eq!(order_update.order_side(), &OrderSide::Buy);
    assert_eq!(
        order_update.order_time_in_force(),
        &TimeInForce::GoodTillCancel
    );
    assert_eq!(order_update.time_expiry(), None);
    assert_eq!(order_update.price(), Decimal::new(105, 1));
    assert_eq!(order_update.quantity(), Decimal::new(12, 1));
}
