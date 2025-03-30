use crate::{ClientError, MessageProcessorError, ServerError, TcpError, ValidationError};
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum StreamError {
    ClientErrorType(ClientError),
    ValidationErrorType(ValidationError),
    DeserializationError(String),
    SerializationError(String),
    ServerErrorType(ServerError),
    TcpErrorType(TcpError),
    MessageProcessorErrorType(MessageProcessorError),
}

impl From<ClientError> for StreamError {
    #[inline]
    fn from(error: ClientError) -> StreamError {
        StreamError::ClientErrorType(error)
    }
}

impl From<MessageProcessorError> for StreamError {
    #[inline]
    fn from(value: MessageProcessorError) -> Self {
        StreamError::MessageProcessorErrorType(value)
    }
}

impl From<ValidationError> for StreamError {
    #[inline]
    fn from(error: ValidationError) -> StreamError {
        StreamError::ValidationErrorType(error)
    }
}

impl From<ServerError> for StreamError {
    #[inline]
    fn from(error: ServerError) -> StreamError {
        StreamError::ServerErrorType(error)
    }
}

impl From<TcpError> for StreamError {
    #[inline]
    fn from(error: TcpError) -> Self {
        StreamError::TcpErrorType(error)
    }
}

impl Error for StreamError {}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::ClientErrorType(err) => write!(f, "ClientError: {}", err),
            StreamError::ValidationErrorType(err) => write!(f, "ValidationError: {}", err),
            StreamError::DeserializationError(err) => write!(f, "DeserializationError: {}", err),
            StreamError::SerializationError(err) => write!(f, "SerializationError: {}", err),
            StreamError::ServerErrorType(err) => write!(f, "ServerError: {}", err),
            StreamError::TcpErrorType(err) => write!(f, "TcpError: {}", err),
            StreamError::MessageProcessorErrorType(err) => {
                write!(f, "MessageProcessorError: {}", err)
            }
        }
    }
}
