use common::prelude::ExchangeID;
use messages::prelude::{MessageType, StopAllDataMessage};

#[test]
fn test_new() {
    let message = StopAllDataMessage::new(ExchangeID::BinanceSpot);
    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_encode() {
    let message = StopAllDataMessage::new(ExchangeID::BinanceSpot);
    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 10);

    let expected: Vec<u8> = vec![2, 0, 3, 0, 1, 0, 1, 0, 3, 1];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![2, 0, 3, 0, 1, 0, 1, 0, 3, 1];
    let buffer = encoded.as_slice();

    let message = StopAllDataMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_message_type() {
    let message = StopAllDataMessage::new(ExchangeID::BinanceSpot);
    assert_eq!(message.message_type(), &MessageType::StopAllData);
}

#[test]
fn test_exchange_id() {
    let message = StopAllDataMessage::new(ExchangeID::BinanceSpot);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_display() {
    let message = StopAllDataMessage::new(ExchangeID::BinanceSpot);

    let expected = "StopAllDataMessage { message_type: StopAllData, exchange_id: BinanceSpot }";
    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}
