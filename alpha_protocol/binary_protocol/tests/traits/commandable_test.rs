use binary_protocol::*;
use rkyv::util::AlignedVec;
use std::fmt::Debug;
use stream_errors::{StreamError, ValidationError};

// Define a simple struct to test the Commandable trait
#[derive(Debug, PartialEq)]
struct TestCommand {
    code: u16,
    data: Vec<u8>,
}

impl TestCommand {
    fn new(code: u16, data: Vec<u8>) -> Self {
        Self { code, data }
    }
}

impl Commandable for TestCommand {
    fn command_code(&self) -> u16 {
        self.code
    }
}

impl BytesSerializable for TestCommand {
    fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
        let mut bytes = AlignedVec::new();
        // Simple serialization: code (2 bytes) + data length (4 bytes) + data
        bytes.extend_from_slice(&self.code.to_le_bytes());
        bytes.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.data);
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, StreamError>
    where
        Self: Sized,
    {
        if bytes.len() < 6 {
            return Err(StreamError::DeserializationError(
                "Not enough bytes for TestCommand".to_string(),
            ));
        }

        let code = u16::from_le_bytes([bytes[0], bytes[1]]);
        let data_len = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]) as usize;

        if bytes.len() < 6 + data_len {
            return Err(StreamError::DeserializationError(
                "Not enough bytes for TestCommand data".to_string(),
            ));
        }

        let data = bytes[6..6 + data_len].to_vec();
        Ok(Self { code, data })
    }
}

impl Sizeable for TestCommand {
    fn byte_size(&self) -> usize {
        // 2 bytes for code + 4 bytes for data length + data length
        2 + 4 + self.data.len()
    }
}

impl Validatable<ValidationError> for TestCommand {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.data.is_empty() {
            return Err(ValidationError::EmptyMessagePayload);
        }
        Ok(())
    }
}

#[test]
fn test_commandable_command_code() {
    let command = TestCommand::new(42, vec![1, 2, 3]);
    assert_eq!(command.command_code(), 42);
}

#[test]
fn test_commandable_with_command_codes() {
    // Test with system command code
    let ping_command = TestCommand::new(PING_CODE, vec![1, 2, 3]);
    assert_eq!(ping_command.command_code(), PING_CODE);

    let pong_command = TestCommand::new(PONG_CODE, vec![1, 2, 3]);
    assert_eq!(pong_command.command_code(), PONG_CODE);

    let heartbeat_command = TestCommand::new(HEARTBEAT_CODE, vec![1, 2, 3]);
    assert_eq!(heartbeat_command.command_code(), HEARTBEAT_CODE);

    // Test with message command code
    let poll_messages_command = TestCommand::new(POLL_MESSAGES_CODE, vec![1, 2, 3]);
    assert_eq!(poll_messages_command.command_code(), POLL_MESSAGES_CODE);
    assert_eq!(poll_messages_command.command_code(), 100);

    // Test with stream command code
    let get_stream_command = TestCommand::new(GET_STREAM_CODE, vec![1, 2, 3]);
    assert_eq!(get_stream_command.command_code(), GET_STREAM_CODE);
    assert_eq!(get_stream_command.command_code(), 200);

    // Test with topic command code
    let get_topic_command = TestCommand::new(GET_TOPIC_CODE, vec![1, 2, 3]);
    assert_eq!(get_topic_command.command_code(), GET_TOPIC_CODE);
    assert_eq!(get_topic_command.command_code(), 300);
}

#[test]
fn test_commandable_serialization() {
    let command = TestCommand::new(42, vec![1, 2, 3]);

    let bytes = command.to_bytes().unwrap();
    let deserialized = TestCommand::from_bytes(&bytes).unwrap();

    assert_eq!(command, deserialized);
    assert_eq!(deserialized.command_code(), 42);
    assert_eq!(deserialized.data, vec![1, 2, 3]);
}

#[test]
fn test_commandable_validation() {
    // Valid command
    let valid_command = TestCommand::new(42, vec![1, 2, 3]);
    assert!(valid_command.validate().is_ok());

    // Invalid command (empty data)
    let invalid_command = TestCommand::new(42, vec![]);
    assert!(invalid_command.validate().is_err());
    assert!(matches!(
        invalid_command.validate().unwrap_err(),
        ValidationError::EmptyMessagePayload
    ));
}

#[test]
fn test_commandable_size() {
    let command = TestCommand::new(42, vec![1, 2, 3]);

    // 2 bytes for code + 4 bytes for data length + 3 bytes for data
    assert_eq!(command.byte_size(), 9);
}

#[test]
fn test_commandable_deserialization_errors() {
    // Not enough bytes for header
    let too_small = vec![0, 1];
    let result = TestCommand::from_bytes(&too_small);
    assert!(result.is_err());

    // Not enough bytes for data
    let missing_data = vec![0, 1, 3, 0, 0, 0]; // Code 1, data length 3, but no data
    let result = TestCommand::from_bytes(&missing_data);
    assert!(result.is_err());
}
