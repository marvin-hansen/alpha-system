use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ClientError {
    UnknownError = 0,
    ConnectionError = 1,
    CannotEstablishConnection = 2,
    NotConnected = 3,
    NotAuthenticated = 4,
    AuthenticationError = 5,
    TcpReadError = 6,
    TcpWriteError = 7,
    TcpFlushError = 8,
    TcpShutdownError = 9,
    ShutdownError = 10,
    InvalidTlsCertificatePath = 11,
    InvalidTlsCertificate = 12,
    InvalidTlsDomain = 13,
    EmptyResponse = 14,
    InvalidNumberEncoding = 15,
    InvalidMessagesCount = 16,
    InvalidClientConfiguration = 17,
    CannotSendMessagesDueToClientDisconnection = 18,
    InvalidMessageFormat = 19,
}

impl Error for ClientError {}

impl From<ClientError> for u8 {
    #[inline]
    fn from(error: ClientError) -> u8 {
        error as u8
    }
}

impl From<u8> for ClientError {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0 => ClientError::UnknownError,
            1 => ClientError::ConnectionError,
            2 => ClientError::CannotEstablishConnection,
            3 => ClientError::NotConnected,
            4 => ClientError::NotAuthenticated,
            5 => ClientError::AuthenticationError,
            6 => ClientError::TcpReadError,
            7 => ClientError::TcpWriteError,
            8 => ClientError::TcpFlushError,
            9 => ClientError::TcpShutdownError,
            10 => ClientError::ShutdownError,
            11 => ClientError::InvalidTlsCertificatePath,
            12 => ClientError::InvalidTlsCertificate,
            13 => ClientError::InvalidTlsDomain,
            14 => ClientError::EmptyResponse,
            15 => ClientError::InvalidNumberEncoding,
            16 => ClientError::InvalidMessagesCount,
            17 => ClientError::InvalidClientConfiguration,
            18 => ClientError::CannotSendMessagesDueToClientDisconnection,
            19 => ClientError::InvalidMessageFormat,
            _ => ClientError::UnknownError,
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::UnknownError => write!(f, "Unknown error"),
            ClientError::ConnectionError => write!(f, "Connection error"),
            ClientError::CannotEstablishConnection => write!(f, "Cannot establish connection"),
            ClientError::NotConnected => write!(f, "Not connected"),
            ClientError::NotAuthenticated => write!(f, "Not authenticated"),
            ClientError::AuthenticationError => write!(f, "Authentication error"),
            ClientError::TcpReadError => write!(f, "TCP read error"),
            ClientError::TcpWriteError => write!(f, "TCP write error"),
            ClientError::TcpFlushError => write!(f, "TCP flush error"),
            ClientError::TcpShutdownError => write!(f, "Tcp shutdown error"),
            ClientError::ShutdownError => write!(f, "Shutdown error"),
            ClientError::InvalidTlsCertificatePath => write!(f, "Invalid TLS certificate path"),
            ClientError::InvalidTlsCertificate => write!(f, "Invalid TLS certificate"),
            ClientError::InvalidTlsDomain => write!(f, "Invalid TLS domain"),
            ClientError::EmptyResponse => write!(f, "Empty response"),
            ClientError::InvalidNumberEncoding => write!(f, "Invalid number encoding"),
            ClientError::InvalidMessagesCount => write!(f, "Invalid messages count"),
            ClientError::InvalidClientConfiguration => write!(f, "Invalid client configuration"),
            ClientError::CannotSendMessagesDueToClientDisconnection => {
                write!(f, "Cannot send messages due to client disconnection")
            }
            ClientError::InvalidMessageFormat => write!(f, "Invalid message format"),
        }
    }
}
