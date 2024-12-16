use common_exchange::ExchangeID;
use common_order::OrderCancelAll;
use sbe_messages_order::SbeOrderCancelAllExtension;

#[test]
fn test_encode_order_cancel_all_message() {
    let cancel_all = OrderCancelAll::new(ExchangeID::Binance, 1);

    let result = cancel_all.encode_to_sbe();
    assert!(result.is_ok());

    let (size, buffer) = result.unwrap();
    assert_eq!(size, 13); // Assert encoded message size matches expected size
    assert!(!buffer.is_empty()); // Assert non-empty encoded message

    let expected: Vec<u8> = vec![5, 0, 148, 1, 1, 0, 1, 0, 148, 1, 4, 1, 0];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode_order_cancel_all_message() {
    let encoded: [u8; 13] = [5, 0, 148, 1, 1, 0, 1, 0, 148, 1, 4, 1, 0];

    let cancel_all = OrderCancelAll::decode_from_sbe(&encoded).expect("Failed to decode");
    assert_eq!(cancel_all.exchange_id(), ExchangeID::Binance);
    assert_eq!(cancel_all.client_id(), 1);
}
