// delta_common/tests/commands/switch_commands/forward_message_test.rs

use binary_protocol::{Commandable, FORWARD_MESSAGES_CODE, ForwardMessage, Sizeable, Validatable};
use stream_errors::ValidationError;

const BASE_SIZE: usize = 4;

const MAX_PAYLOAD_SIZE: u32 = 1024 * 1024; // 1 MB

#[test]
fn test_forward_message_new() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload.clone());

    assert_eq!(message.payload(), &payload);
    assert_eq!(message.cached_payload_size(), payload.len() as u32);
}

#[test]
fn test_forward_message_messages() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload.clone());

    assert_eq!(message.payload(), &payload);
}

#[test]
fn test_forward_message_cached_payload_size() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload);

    assert_eq!(message.cached_payload_size(), 4);
}

#[test]
fn test_forward_message_command_code() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload);

    assert_eq!(message.command_code(), FORWARD_MESSAGES_CODE);
}

#[test]
fn test_forward_message_validate_success() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload);

    assert!(message.validate().is_ok());
}

#[test]
fn test_forward_message_validate_empty_payload() {
    let payload = vec![];
    let message = ForwardMessage::new(payload);

    assert_eq!(
        message.validate().unwrap_err(),
        ValidationError::EmptyMessagesCount
    );
}

#[test]
fn test_forward_message_validate_too_big_payload() {
    let payload = vec![0; (MAX_PAYLOAD_SIZE + 1) as usize];
    let message = ForwardMessage::new(payload);

    assert_eq!(
        message.validate().unwrap_err(),
        ValidationError::TooBigMessagePayload(MAX_PAYLOAD_SIZE + 1, MAX_PAYLOAD_SIZE)
    );
}

#[test]
fn test_forward_message_byte_size() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload);

    assert_eq!(message.byte_size(), 4 + BASE_SIZE);
}

#[test]
fn test_forward_message_display() {
    let payload = vec![1, 2, 3, 4];
    let message = ForwardMessage::new(payload);

    assert_eq!(format!("{}", message), "ForwardMessage");
}
