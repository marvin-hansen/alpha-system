use common::prelude::MessageClientConfig;
use sbe_messages::prelude::{ClientLoginMessage, MessageType};

#[test]
fn test_new() {
    let client_id = 100;
    let client_name = "client-100".to_string();
    let config = MessageClientConfig::new(client_id, client_name.clone());
    let message = ClientLoginMessage::from_config(&config);

    assert_eq!(message.message_type(), &MessageType::ClientLogin);
    assert_eq!(message.client_id(), client_id);
    assert_eq!(message.client_name(), client_name.clone());
}

#[test]
fn test_encode() {
    let client_id = 100;
    let client_name: [u8; 10] = "client-100".as_bytes().try_into().unwrap();
    let message = ClientLoginMessage::new(client_id, client_name);

    assert_eq!(message.message_type(), &MessageType::ClientLogin);
    assert_eq!(message.client_id(), client_id);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 21);

    let expected: Vec<u8> = vec![
        13, 0, 1, 0, 1, 0, 1, 0, 1, 100, 0, 99, 108, 105, 101, 110, 116, 45, 49, 48, 48,
    ];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![
        13, 0, 1, 0, 1, 0, 1, 0, 1, 100, 0, 99, 108, 105, 101, 110, 116, 45, 49, 48, 48,
    ];
    let buffer = encoded.as_slice();

    let message = ClientLoginMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::ClientLogin);
    assert_eq!(message.client_id(), 100);
}

#[test]
fn test_message_type() {
    let client_id = 100;
    let client_name: [u8; 10] = "client-100".as_bytes().try_into().unwrap();
    let message = ClientLoginMessage::new(client_id, client_name);

    assert_eq!(message.message_type(), &MessageType::ClientLogin);
}

#[test]
fn test_message_client_id() {
    let client_id = 100;
    let client_name: [u8; 10] = "client-100".as_bytes().try_into().unwrap();
    let message = ClientLoginMessage::new(client_id, client_name);

    assert_eq!(message.client_id(), client_id);
}

#[test]
fn test_message_client_name() {
    let client_id = 100;
    let client_name: [u8; 10] = "client-100".as_bytes().try_into().unwrap();
    let message = ClientLoginMessage::new(client_id, client_name);

    // convert client_name to String
    let client_name = String::from_utf8(client_name.to_vec()).unwrap();

    assert_eq!(message.client_name(), client_name);
}

#[test]
fn test_display() {
    let client_id = 100;
    let client_name: [u8; 10] = "client-100".as_bytes().try_into().unwrap();

    let actual = ClientLoginMessage::new(client_id, client_name);

    let expected = "ClientLoginMessage { client_id: 100, client_name: client-100 }";

    assert_eq!(format!("{}", actual), expected);
}
