use std::error::Error;
use stream_errors::ClientError;

#[test]
fn test_client_error_from_u8() {
    // Test all valid conversions
    assert_eq!(ClientError::from(0), ClientError::UnknownError);
    assert_eq!(ClientError::from(1), ClientError::ConnectionError);
    assert_eq!(ClientError::from(2), ClientError::CannotEstablishConnection);
    assert_eq!(ClientError::from(3), ClientError::NotConnected);
    assert_eq!(ClientError::from(4), ClientError::NotAuthenticated);
    assert_eq!(ClientError::from(5), ClientError::AuthenticationError);
    assert_eq!(ClientError::from(6), ClientError::TcpReadError);
    assert_eq!(ClientError::from(7), ClientError::TcpWriteError);
    assert_eq!(ClientError::from(8), ClientError::TcpFlushError);
    assert_eq!(ClientError::from(9), ClientError::TcpShutdownError);
    assert_eq!(ClientError::from(10), ClientError::ShutdownError);
    assert_eq!(
        ClientError::from(11),
        ClientError::InvalidTlsCertificatePath
    );
    assert_eq!(ClientError::from(12), ClientError::InvalidTlsCertificate);
    assert_eq!(ClientError::from(13), ClientError::InvalidTlsDomain);
    assert_eq!(ClientError::from(14), ClientError::EmptyResponse);
    assert_eq!(ClientError::from(15), ClientError::InvalidNumberEncoding);

    // Test invalid value defaults to UnknownError
    assert_eq!(ClientError::from(255), ClientError::UnknownError);
}

#[test]
fn test_client_error_to_u8() {
    // Test all conversions
    assert_eq!(u8::from(ClientError::UnknownError), 0);
    assert_eq!(u8::from(ClientError::ConnectionError), 1);
    assert_eq!(u8::from(ClientError::CannotEstablishConnection), 2);
    assert_eq!(u8::from(ClientError::NotConnected), 3);
    assert_eq!(u8::from(ClientError::NotAuthenticated), 4);
    assert_eq!(u8::from(ClientError::AuthenticationError), 5);
    assert_eq!(u8::from(ClientError::TcpReadError), 6);
    assert_eq!(u8::from(ClientError::TcpWriteError), 7);
    assert_eq!(u8::from(ClientError::TcpFlushError), 8);
    assert_eq!(u8::from(ClientError::TcpShutdownError), 9);
    assert_eq!(u8::from(ClientError::ShutdownError), 10);
    assert_eq!(u8::from(ClientError::InvalidTlsCertificatePath), 11);
    assert_eq!(u8::from(ClientError::InvalidTlsCertificate), 12);
    assert_eq!(u8::from(ClientError::InvalidTlsDomain), 13);
    assert_eq!(u8::from(ClientError::EmptyResponse), 14);
    assert_eq!(u8::from(ClientError::InvalidNumberEncoding), 15);
}

#[test]
fn test_client_error_display() {
    // Test display implementation for each variant
    assert_eq!(ClientError::UnknownError.to_string(), "Unknown error");
    assert_eq!(ClientError::ConnectionError.to_string(), "Connection error");
    assert_eq!(
        ClientError::CannotEstablishConnection.to_string(),
        "Cannot establish connection"
    );
    assert_eq!(ClientError::NotConnected.to_string(), "Not connected");
    assert_eq!(
        ClientError::NotAuthenticated.to_string(),
        "Not authenticated"
    );
    assert_eq!(
        ClientError::AuthenticationError.to_string(),
        "Authentication error"
    );
    assert_eq!(ClientError::TcpReadError.to_string(), "TCP read error");
    assert_eq!(ClientError::TcpWriteError.to_string(), "TCP write error");
    assert_eq!(ClientError::TcpFlushError.to_string(), "TCP flush error");
    assert_eq!(
        ClientError::TcpShutdownError.to_string(),
        "Tcp shutdown error"
    );
    assert_eq!(ClientError::ShutdownError.to_string(), "Shutdown error");
    assert_eq!(
        ClientError::InvalidTlsCertificatePath.to_string(),
        "Invalid TLS certificate path"
    );
    assert_eq!(
        ClientError::InvalidTlsCertificate.to_string(),
        "Invalid TLS certificate"
    );
    assert_eq!(
        ClientError::InvalidTlsDomain.to_string(),
        "Invalid TLS domain"
    );
    assert_eq!(ClientError::EmptyResponse.to_string(), "Empty response");
    assert_eq!(
        ClientError::InvalidNumberEncoding.to_string(),
        "Invalid number encoding"
    );
}

#[test]
fn test_client_error_debug() {
    // Test debug formatting
    assert!(format!("{:?}", ClientError::UnknownError).contains("UnknownError"));
    assert!(format!("{:?}", ClientError::ConnectionError).contains("ConnectionError"));
}

#[test]
fn test_client_error_clone() {
    // Test cloning
    let error = ClientError::UnknownError;
    let cloned = error;
    assert_eq!(error, cloned);
}

#[test]
fn test_client_error_copy() {
    // Test copy trait
    let error = ClientError::UnknownError;
    let copied = error;
    assert_eq!(error, copied);
}

#[test]
fn test_client_error_partial_eq() {
    // Test equality
    assert_eq!(ClientError::UnknownError, ClientError::UnknownError);
    assert_ne!(ClientError::UnknownError, ClientError::ConnectionError);
}

#[test]
fn test_client_error_implements_error_trait() {
    // Test that ClientError implements Error trait
    fn requires_error(_: &dyn Error) {}
    requires_error(&ClientError::UnknownError);
}
