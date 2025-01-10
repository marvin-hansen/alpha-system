use common_exchange::ExchangeID;
use common_order::{ClientOrderID, ExchangeOrderID, OrderCancel};
use sbe_messages_order::{decode_order_cancel_message, encode_order_cancel_message};

#[test]
fn test_encode_order_cancel_message() {
    let cancel_order = OrderCancel::new(
        ExchangeID::BinanceSpot,
        1,
        ClientOrderID::from("clt_ord_id"),
        ExchangeOrderID::from("exchange_order_id"),
    );

    let result = encode_order_cancel_message(cancel_order);
    assert!(result.is_ok());

    let (size, buffer) = result.unwrap();
    assert_eq!(size, 37); // Assert encoded message size matches expected
    assert!(!buffer.is_empty()); // Assert non-empty encoded message

    let expected: Vec<u8> = vec![
        29, 0, 147, 1, 1, 0, 1, 0, 147, 1, 4, 1, 0, 228, 247, 147, 162, 247, 115, 229, 9, 91, 124,
        142, 29, 200, 110, 191, 7, 34, 185, 137, 255, 71, 2, 0, 0,
    ];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode_order_cancel_message() {
    let encoded: Vec<u8> = vec![
        29, 0, 147, 1, 1, 0, 1, 0, 147, 1, 4, 1, 0, 228, 247, 147, 162, 247, 115, 229, 9, 91, 124,
        142, 29, 200, 110, 191, 7, 34, 185, 137, 255, 71, 2, 0, 0,
    ];

    let cancel_order = decode_order_cancel_message(encoded.as_slice());
    assert!(cancel_order.is_ok());
    let cancel_order = cancel_order.unwrap();
    assert_eq!(cancel_order.exchange_id(), ExchangeID::BinanceSpot);
    assert_eq!(cancel_order.client_id(), 1);
    assert_eq!(
        cancel_order.client_order_id(),
        &ClientOrderID::from("clt_ord_id")
    );
    assert_eq!(
        cancel_order.exchange_order_id(),
        &ExchangeOrderID::from("exchange_order_id")
    );
}
