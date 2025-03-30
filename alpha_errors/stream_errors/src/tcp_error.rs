use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TcpError {
    UnknownTcpError = 0,
    FailedToSetTcpNodelay = 1,
    FailedToSetSoKeepAlive = 2,
    FailedToSetSoRcvbuf = 4,
    FailedToSetSoSndbuf = 5,
    FailedToSetSoReuseaddr = 6,
    FailedToSetSoReuseport = 7,
}

impl Error for TcpError {}

impl From<u8> for TcpError {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => TcpError::UnknownTcpError,
            1 => TcpError::FailedToSetTcpNodelay,
            2 => TcpError::FailedToSetSoKeepAlive,
            4 => TcpError::FailedToSetSoRcvbuf,
            5 => TcpError::FailedToSetSoSndbuf,
            6 => TcpError::FailedToSetSoReuseaddr,
            7 => TcpError::FailedToSetSoReuseport,
            _ => TcpError::UnknownTcpError,
        }
    }
}

impl From<TcpError> for u8 {
    #[inline]
    fn from(error: TcpError) -> u8 {
        error as u8
    }
}

impl std::fmt::Display for TcpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TcpError::UnknownTcpError => write!(f, "unknown TCP error"),
            TcpError::FailedToSetTcpNodelay => write!(f, "failed to set TCP NO_DELAY"),
            TcpError::FailedToSetSoKeepAlive => write!(f, "failed to set SO_KEEPALIVE"),
            TcpError::FailedToSetSoReuseaddr => write!(f, "failed to set SO_REUSEADDR"),
            TcpError::FailedToSetSoRcvbuf => write!(f, "failed to set SO_RCVBUF"),
            TcpError::FailedToSetSoSndbuf => write!(f, "failed to set SO_SNDBUF"),
            TcpError::FailedToSetSoReuseport => write!(f, "failed to set SO_REUSEPORT"),
        }
    }
}
