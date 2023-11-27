use common::prelude::{ExchangeID, SymbolID};
use messages::prelude::{MessageType, StopDataMessage};

#[test]
fn test_new() {
    let message = StopDataMessage::new(ExchangeID::BNB, SymbolID::BTCUSD);
    assert_eq!(message.message_type(), &MessageType::StopData);
    assert_eq!(message.exchange_id(), &ExchangeID::BNB);
    assert_eq!(message.symbol_id(), &SymbolID::BTCUSD);
}

#[test]
fn test_encode() {
    let message = StopDataMessage::new(ExchangeID::BNB, SymbolID::BTCUSD);
    assert_eq!(message.message_type(), &MessageType::StopData);
    assert_eq!(message.exchange_id(), &ExchangeID::BNB);
    assert_eq!(message.symbol_id(), &SymbolID::BTCUSD);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 2, 0, 1, 0, 1, 0, 2, 1, 1, 0];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 2, 0, 1, 0, 1, 0, 2, 1, 1, 0];
    let buffer = encoded.as_slice();

    let message = StopDataMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::StopData);
    assert_eq!(message.exchange_id(), &ExchangeID::BNB);
    assert_eq!(message.symbol_id(), &SymbolID::BTCUSD);
}

#[test]
fn test_message_type() {
    let message = StopDataMessage::new(ExchangeID::BNB, SymbolID::BTCUSD);
    assert_eq!(message.message_type(), &MessageType::StopData);
}

#[test]
fn test_exchange_id() {
    let message = StopDataMessage::new(ExchangeID::BNB, SymbolID::BTCUSD);
    assert_eq!(message.exchange_id(), &ExchangeID::BNB);
}

#[test]
fn test_symbol_id() {
    let message = StopDataMessage::new(ExchangeID::BNB, SymbolID::BTCUSD);
    assert_eq!(message.symbol_id(), &SymbolID::BTCUSD);
}

#[test]
fn test_display() {
    let message = StopDataMessage::new(ExchangeID::BNB, SymbolID::BTCUSD);

    let expected =
        "StopDataMessage { message_type: StopData, exchange_id: BNB, symbol_id: BTCUSD }";
    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}
