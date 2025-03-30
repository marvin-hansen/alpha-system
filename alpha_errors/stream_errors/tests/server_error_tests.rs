use std::error::Error;
use stream_errors::ServerError;

#[test]
fn test_server_error_from_u8() {
    // Test all valid conversions
    assert_eq!(ServerError::from(99u8), ServerError::UnknownServerError);
    assert_eq!(ServerError::from(1u8), ServerError::TopicAlreadyExists);
    assert_eq!(ServerError::from(2u8), ServerError::StreamAlreadyExists);
    assert_eq!(ServerError::from(3u8), ServerError::UserAlreadyExists);
    assert_eq!(
        ServerError::from(4u8),
        ServerError::AccessTokenAlreadyExists
    );
    assert_eq!(ServerError::from(5u8), ServerError::TopicNotFound);
    assert_eq!(ServerError::from(6u8), ServerError::StreamNotFound);
    assert_eq!(ServerError::from(7u8), ServerError::UserNotFound);
    assert_eq!(ServerError::from(8u8), ServerError::AccessTokenNotFound);
    assert_eq!(ServerError::from(9u8), ServerError::InvalidCredentials);
    assert_eq!(ServerError::from(10u8), ServerError::InvalidTopicId);
    assert_eq!(ServerError::from(11u8), ServerError::InvalidStreamId);
    assert_eq!(ServerError::from(12u8), ServerError::InvalidAccessToken);
    assert_eq!(ServerError::from(13u8), ServerError::InvalidMessageId);
    assert_eq!(ServerError::from(14u8), ServerError::InvalidMessagePayload);

    // Test invalid value defaults to UnknownServerError
    assert_eq!(ServerError::from(255u8), ServerError::UnknownServerError);
}

#[test]
fn test_server_error_from_u16() {
    // Test conversions from u16
    assert_eq!(ServerError::from(99u16), ServerError::UnknownServerError);
    assert_eq!(ServerError::from(1u16), ServerError::TopicAlreadyExists);

    // Test larger u16 value
    assert_eq!(ServerError::from(300u16), ServerError::UnknownServerError);
}

#[test]
fn test_server_error_from_u32() {
    // Test conversions from u32
    assert_eq!(ServerError::from(99u32), ServerError::UnknownServerError);
    assert_eq!(ServerError::from(1u32), ServerError::TopicAlreadyExists);

    // Test larger u32 value
    assert_eq!(ServerError::from(65536u32), ServerError::UnknownServerError);
}

#[test]
fn test_server_error_to_u8() {
    // Test all conversions
    assert_eq!(u8::from(ServerError::UnknownServerError), 99);
    assert_eq!(u8::from(ServerError::TopicAlreadyExists), 1);
    assert_eq!(u8::from(ServerError::StreamAlreadyExists), 2);
    assert_eq!(u8::from(ServerError::UserAlreadyExists), 3);
    assert_eq!(u8::from(ServerError::AccessTokenAlreadyExists), 4);
    assert_eq!(u8::from(ServerError::TopicNotFound), 5);
    assert_eq!(u8::from(ServerError::StreamNotFound), 6);
    assert_eq!(u8::from(ServerError::UserNotFound), 7);
    assert_eq!(u8::from(ServerError::AccessTokenNotFound), 8);
    assert_eq!(u8::from(ServerError::InvalidCredentials), 9);
    assert_eq!(u8::from(ServerError::InvalidTopicId), 10);
    assert_eq!(u8::from(ServerError::InvalidStreamId), 11);
    assert_eq!(u8::from(ServerError::InvalidAccessToken), 12);
    assert_eq!(u8::from(ServerError::InvalidMessageId), 13);
    assert_eq!(u8::from(ServerError::InvalidMessagePayload), 14);
}

#[test]
fn test_server_error_display() {
    // Test display implementation for each variant
    assert_eq!(
        ServerError::UnknownServerError.to_string(),
        "unknown server error"
    );
    assert_eq!(
        ServerError::TopicAlreadyExists.to_string(),
        "topic already exists"
    );
    assert_eq!(
        ServerError::StreamAlreadyExists.to_string(),
        "stream already exists"
    );
    assert_eq!(
        ServerError::UserAlreadyExists.to_string(),
        "user already exists"
    );
    assert_eq!(
        ServerError::AccessTokenAlreadyExists.to_string(),
        "access token already exists"
    );
    assert_eq!(ServerError::TopicNotFound.to_string(), "topic not found");
    assert_eq!(ServerError::StreamNotFound.to_string(), "stream not found");
    assert_eq!(ServerError::UserNotFound.to_string(), "user not found");
    assert_eq!(
        ServerError::AccessTokenNotFound.to_string(),
        "access token not found"
    );
    assert_eq!(
        ServerError::InvalidCredentials.to_string(),
        "invalid credentials"
    );
    assert_eq!(ServerError::InvalidTopicId.to_string(), "invalid topic id");
    assert_eq!(
        ServerError::InvalidStreamId.to_string(),
        "invalid stream id"
    );
    assert_eq!(
        ServerError::InvalidAccessToken.to_string(),
        "invalid access token"
    );
    assert_eq!(
        ServerError::InvalidMessageId.to_string(),
        "invalid message id"
    );
    assert_eq!(
        ServerError::InvalidMessagePayload.to_string(),
        "invalid message payload"
    );
}

#[test]
fn test_server_error_debug() {
    // Test debug formatting
    assert!(format!("{:?}", ServerError::UnknownServerError).contains("UnknownServerError"));
    assert!(format!("{:?}", ServerError::TopicAlreadyExists).contains("TopicAlreadyExists"));
}

#[test]
fn test_server_error_clone() {
    // Test cloning
    let error = ServerError::UnknownServerError;
    let cloned = error;
    assert_eq!(error, cloned);
}

#[test]
fn test_server_error_partial_eq() {
    // Test equality
    assert_eq!(
        ServerError::UnknownServerError,
        ServerError::UnknownServerError
    );
    assert_ne!(
        ServerError::UnknownServerError,
        ServerError::TopicAlreadyExists
    );
}

#[test]
fn test_server_error_implements_error_trait() {
    // Test that ServerError implements Error trait
    fn requires_error(_: &dyn Error) {}
    requires_error(&ServerError::UnknownServerError);
}
