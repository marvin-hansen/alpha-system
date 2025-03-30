use binary_protocol::{
    BytesSerializable, Commandable, REGISTER_CLIENT_CODE, RegisterClient, Sizeable, Validatable,
};
use std::format;
use stream_errors::ValidationError;

#[test]
fn test_new_register_client() {
    let client_id = 123;
    let register_client = RegisterClient::new(client_id);
    assert_eq!(register_client.client_id(), client_id);
}

#[test]
fn test_register_client_command_code() {
    let register_client = RegisterClient::new(123);
    assert_eq!(register_client.command_code(), REGISTER_CLIENT_CODE);
}

#[test]
fn test_register_client_byte_size() {
    let register_client = RegisterClient::new(123);
    assert_eq!(register_client.byte_size(), 2);
}

#[test]
fn test_register_client_validation() {
    let valid_client = RegisterClient::new(123);
    assert!(valid_client.validate().is_ok());

    let invalid_client = RegisterClient::new(0);
    assert_eq!(
        invalid_client.validate(),
        Err(ValidationError::ClientIdMustNotBeNull)
    );
}

#[test]
fn test_register_client_display() {
    let client_id = 123;
    let register_client = RegisterClient::new(client_id);
    let display = format!("{}", register_client);
    assert_eq!(display, "RegisterClient { client_id: 123 }");
}

#[test]
fn test_register_client_serialization() {
    let client_id = 123;
    let register_client = RegisterClient::new(client_id);
    let serialized = register_client.to_bytes().unwrap();
    let deserialized = RegisterClient::from_bytes(&serialized).unwrap();
    assert_eq!(register_client, deserialized);
}
