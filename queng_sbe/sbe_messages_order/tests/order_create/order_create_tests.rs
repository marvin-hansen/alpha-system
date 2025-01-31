/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_exchange::ExchangeID;
use common_order::{OrderCreate, OrderSide, OrderType, TimeInForce};
use rust_decimal::Decimal;
use sbe_messages_order::{decode_order_create_message, encode_order_create_message};

#[test]
fn test_encode_order_create_message() {
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

    let result = encode_order_create_message(order);
    assert!(result.is_ok());

    let (size, buffer) = result.unwrap();
    assert_eq!(size, 75); // Assert encoded message size matches expected
    assert!(!buffer.is_empty()); // Assert non-empty encoded message

    let expected: Vec<u8> = vec![
        67, 0, 145, 1, 1, 0, 1, 0, 145, 1, 4, 1, 0, 228, 247, 147, 162, 247, 151, 39, 0, 129, 144,
        48, 135, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 9, 179, 97, 0, 0, 0, 0, 20, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 163, 225, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode_order_create_message() {
    let encoded: Vec<u8> = vec![
        67, 0, 145, 1, 1, 0, 1, 0, 145, 1, 4, 1, 0, 228, 247, 147, 162, 247, 151, 39, 0, 129, 144,
        48, 135, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 9, 179, 97, 0, 0, 0, 0, 20, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 163, 225, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let order = decode_order_create_message(encoded.as_slice());
    assert!(order.is_ok());

    let order = order.unwrap();
    assert_eq!(order.exchange_id(), ExchangeID::BinanceSpot);
    assert_eq!(order.client_id(), 1);
    assert_eq!(order.client_order_id().client_order_id(), "cl_ord_id");
    assert_eq!(order.symbol_id_exchange().exchange_order_id(), "ETHUSD");
    assert_eq!(order.order_type(), &OrderType::Market);
    assert_eq!(order.order_side(), &OrderSide::Sell);
    assert_eq!(
        order.order_time_in_force(),
        &TimeInForce::GoodTillTimeExchange
    );
    assert_eq!(order.time_expiry(), Some(1639123200));

    assert_eq!(order.quantity(), Decimal::new(20, 1));
    assert_eq!(order.price(), Decimal::new(300000000, 1));
}
