use std::error::Error;
use stream_errors::{ClientError, ServerError, StreamError, ValidationError};

#[test]
fn test_stream_error_from_client_error() {
    let client_error = ClientError::ConnectionError;
    let stream_error = StreamError::from(client_error);

    match stream_error {
        StreamError::ClientErrorType(err) => assert_eq!(err, ClientError::ConnectionError),
        _ => panic!("Expected ClientErrorType variant"),
    }
}

#[test]
fn test_stream_error_from_validation_error() {
    let validation_error = ValidationError::EmptyMessagePayload;
    let stream_error = StreamError::from(validation_error);

    match stream_error {
        StreamError::ValidationErrorType(err) => {
            assert_eq!(err, ValidationError::EmptyMessagePayload)
        }
        _ => panic!("Expected ValidationErrorType variant"),
    }
}

#[test]
fn test_stream_error_from_server_error() {
    let server_error = ServerError::TopicNotFound;
    let stream_error = StreamError::from(server_error);

    match stream_error {
        StreamError::ServerErrorType(err) => assert_eq!(err, ServerError::TopicNotFound),
        _ => panic!("Expected ServerErrorType variant"),
    }
}

#[test]
fn test_stream_error_deserialization_error() {
    let error_message = "Failed to deserialize JSON";
    let stream_error = StreamError::DeserializationError(error_message.to_string());

    match stream_error {
        StreamError::DeserializationError(msg) => assert_eq!(msg, error_message),
        _ => panic!("Expected DeserializationError variant"),
    }
}

#[test]
fn test_stream_error_serialization_error() {
    let error_message = "Failed to serialize to JSON";
    let stream_error = StreamError::SerializationError(error_message.to_string());

    match stream_error {
        StreamError::SerializationError(msg) => assert_eq!(msg, error_message),
        _ => panic!("Expected SerializationError variant"),
    }
}

#[test]
fn test_stream_error_display() {
    // Test display implementation for each variant
    let client_error = ClientError::ConnectionError;
    let validation_error = ValidationError::EmptyMessagePayload;
    let server_error = ServerError::TopicNotFound;

    let client_stream_error = StreamError::from(client_error);
    let validation_stream_error = StreamError::from(validation_error);
    let server_stream_error = StreamError::from(server_error);
    let deserialization_error = StreamError::DeserializationError("Deser error".to_string());
    let serialization_error = StreamError::SerializationError("Ser error".to_string());

    assert_eq!(
        client_stream_error.to_string(),
        "ClientError: Connection error"
    );
    assert_eq!(
        validation_stream_error.to_string(),
        "ValidationError: EmptyMessagePayload"
    );
    assert_eq!(
        server_stream_error.to_string(),
        "ServerError: topic not found"
    );
    assert_eq!(
        deserialization_error.to_string(),
        "DeserializationError: Deser error"
    );
    assert_eq!(
        serialization_error.to_string(),
        "SerializationError: Ser error"
    );
}

#[test]
fn test_stream_error_debug() {
    // Test debug formatting
    let client_error = StreamError::from(ClientError::ConnectionError);
    assert!(format!("{:?}", client_error).contains("ClientErrorType"));

    let deser_error = StreamError::DeserializationError("test".to_string());
    assert!(format!("{:?}", deser_error).contains("DeserializationError"));
}

#[test]
fn test_stream_error_clone() {
    // Test cloning
    let error = StreamError::from(ClientError::ConnectionError);
    let cloned = error.clone();
    assert_eq!(error, cloned);

    let error = StreamError::DeserializationError("test".to_string());
    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn test_stream_error_partial_eq() {
    // Test equality
    let error1 = StreamError::from(ClientError::ConnectionError);
    let error2 = StreamError::from(ClientError::ConnectionError);
    let error3 = StreamError::from(ClientError::NotConnected);

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);

    let error4 = StreamError::DeserializationError("test".to_string());
    let error5 = StreamError::DeserializationError("test".to_string());
    let error6 = StreamError::DeserializationError("different".to_string());

    assert_eq!(error4, error5);
    assert_ne!(error4, error6);
}

#[test]
fn test_stream_error_implements_error_trait() {
    // Test that StreamError implements Error trait
    fn requires_error(_: &dyn Error) {}
    requires_error(&StreamError::from(ClientError::ConnectionError));
    requires_error(&StreamError::DeserializationError("test".to_string()));
}
