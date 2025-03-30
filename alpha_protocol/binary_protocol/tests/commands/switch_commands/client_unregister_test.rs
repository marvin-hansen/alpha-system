use binary_protocol::{
    BytesSerializable, Commandable, Sizeable, UNREGISTER_CLIENT_CODE, UnRegisterClient, Validatable,
};
use stream_errors::ValidationError;

#[test]
fn test_new_unregister_client() {
    let client_id = 123;
    let unregister_client = UnRegisterClient::new(client_id);
    assert_eq!(unregister_client.client_id(), client_id);
}

#[test]
fn test_command_code() {
    let unregister_client = UnRegisterClient::new(123);
    assert_eq!(unregister_client.command_code(), UNREGISTER_CLIENT_CODE);
}

#[test]
fn test_byte_size() {
    let unregister_client = UnRegisterClient::new(123);
    assert_eq!(unregister_client.byte_size(), 2);
}

#[test]
fn test_validate_valid_client_id() {
    let unregister_client = UnRegisterClient::new(123);
    assert!(unregister_client.validate().is_ok());
}

#[test]
fn test_validate_invalid_client_id() {
    let unregister_client = UnRegisterClient::new(0);
    assert_eq!(
        unregister_client.validate(),
        Err(ValidationError::ClientIdMustNotBeNull)
    );
}

#[test]
fn test_display() {
    let client_id = 123;
    let unregister_client = UnRegisterClient::new(client_id);
    assert_eq!(
        format!("{}", unregister_client),
        format!("UnRegisterClient {{ client_id: {} }}", client_id)
    );
}

#[test]
fn test_unregister_client_serialization() {
    let client_id = 123;
    let unregister_client = UnRegisterClient::new(client_id);
    let serialized = unregister_client.to_bytes().unwrap();
    let deserialized = UnRegisterClient::from_bytes(&serialized).unwrap();
    assert_eq!(unregister_client, deserialized);
}
