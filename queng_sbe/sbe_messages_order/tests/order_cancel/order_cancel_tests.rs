use common_exchange::ExchangeID;
use common_order::OrderCancel;
use sbe_messages_order::SbeOrderCancelExtension;

#[test]
fn test_encode_order_cancel_message() {
    let cancel_order = OrderCancel::new(
        ExchangeID::Binance,
        1,
        "clt_order_id".to_string(),
        "exchange_order_id".to_string(),
    );

    let result = cancel_order.encode_to_sbe();
    assert!(result.is_ok());

    let (size, buffer) = result.unwrap();
    assert_eq!(size, 47); // Assert encoded message size matches expected
    assert!(!buffer.is_empty()); // Assert non-empty encoded message

    let expected: Vec<u8> = vec![
        39, 0, 147, 1, 1, 0, 1, 0, 147, 1, 4, 1, 0, 99, 108, 116, 95, 111, 114, 100, 101, 114, 95,
        105, 100, 0, 0, 101, 120, 99, 104, 97, 110, 103, 101, 95, 111, 114, 100, 101, 114, 95, 105,
        100, 0, 0, 0,
    ];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode_order_cancel_message() {
    let encoded: Vec<u8> = vec![
        39, 0, 147, 1, 1, 0, 1, 0, 147, 1, 4, 1, 0, 99, 108, 116, 95, 111, 114, 100, 101, 114, 95,
        105, 100, 0, 0, 101, 120, 99, 104, 97, 110, 103, 101, 95, 111, 114, 100, 101, 114, 95, 105,
        100, 0, 0, 0,
    ];

    let cancel_order = OrderCancel::decode_from_sbe(encoded.as_slice()).expect("Failed to decode");
    assert_eq!(cancel_order.exchange_id(), ExchangeID::Binance);
    assert_eq!(cancel_order.client_id(), 1);
    assert_eq!(cancel_order.client_order_id(), "clt_order_id");
    assert_eq!(cancel_order.exchange_order_id(), "exchange_order_id");
}
