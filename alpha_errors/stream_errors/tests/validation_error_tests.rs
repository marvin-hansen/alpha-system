use std::error::Error;
use stream_errors::ValidationError;

#[test]
fn test_validation_error_empty_message_payload() {
    let error = ValidationError::EmptyMessagePayload;
    assert_eq!(error.to_string(), "EmptyMessagePayload");
}

#[test]
fn test_validation_error_empty_messages_count() {
    let error = ValidationError::EmptyMessagesCount;
    assert_eq!(error.to_string(), "EmptyMessagesCount");
}

#[test]
fn test_validation_error_too_big_message_payload() {
    let size = 1024;
    let max = 512;
    let error = ValidationError::TooBigMessagePayload(size, max);
    assert_eq!(
        error.to_string(),
        "TooBigMessagePayload: 1024 Maximum allowed size: 512"
    );
}

#[test]
fn test_validation_error_too_long_string() {
    let field = "username".to_string();
    let size = 20;
    let max = 10;
    let error = ValidationError::TooLongString(field, size, max);
    assert_eq!(
        error.to_string(),
        "TooLongString: username has size: 20. Maximum allowed size: 10"
    );
}

#[test]
fn test_validation_error_payload_checksum_verification_failed() {
    let error = ValidationError::PayloadChecksumVerificationFailed;
    assert_eq!(error.to_string(), "PayloadChecksumVerificationFailed");
}

#[test]
fn test_validation_error_payload_too_long() {
    let error = ValidationError::PayloadTooLong;
    assert_eq!(error.to_string(), "PayloadTooLong");
}

#[test]
fn test_validation_error_debug() {
    // Test debug formatting
    assert!(format!("{:?}", ValidationError::EmptyMessagePayload).contains("EmptyMessagePayload"));

    let size = 1024;
    let max = 512;
    let debug_str = format!("{:?}", ValidationError::TooBigMessagePayload(size, max));
    assert!(debug_str.contains("TooBigMessagePayload"));
    assert!(debug_str.contains("1024"));
    assert!(debug_str.contains("512"));
}

#[test]
fn test_validation_error_clone() {
    // Test cloning
    let error = ValidationError::EmptyMessagePayload;
    let cloned = error.clone();
    assert_eq!(error, cloned);

    let size = 1024;
    let max = 512;
    let error = ValidationError::TooBigMessagePayload(size, max);
    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn test_validation_error_partial_eq() {
    // Test equality
    assert_eq!(
        ValidationError::EmptyMessagePayload,
        ValidationError::EmptyMessagePayload
    );
    assert_ne!(
        ValidationError::EmptyMessagePayload,
        ValidationError::EmptyMessagesCount
    );

    let error1 = ValidationError::TooBigMessagePayload(1024, 512);
    let error2 = ValidationError::TooBigMessagePayload(1024, 512);
    let error3 = ValidationError::TooBigMessagePayload(2048, 512);

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_validation_error_implements_error_trait() {
    // Test that ValidationError implements Error trait
    fn requires_error(_: &dyn Error) {}
    requires_error(&ValidationError::EmptyMessagePayload);
}
