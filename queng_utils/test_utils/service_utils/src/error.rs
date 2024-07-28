use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServiceUtilError {
    ServiceStartFailed(String),
    ServiceStopFailed(String),
    ServiceNotSupported(String),
    ServiceNotRunning(String),
    UnknownError(String),
}

impl Error for ServiceUtilError {}

impl fmt::Display for ServiceUtilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ServiceUtilError::ServiceStartFailed(e) => {
                write!(f, "[ServiceUtilError]: Service start failed: {e}")
            }
            ServiceUtilError::ServiceStopFailed(e) => {
                write!(f, "[ServiceUtilError]: Service stop failed: {e}")
            }
            ServiceUtilError::ServiceNotSupported(e) => {
                write!(f, "[ServiceUtilError]: Service not supported: {e}")
            }
            ServiceUtilError::ServiceNotRunning(e) => {
                write!(f, "[ServiceUtilError]: Service not running: {e}")
            }
            ServiceUtilError::UnknownError(e) => {
                write!(f, "[ServiceUtilError]: Unknown error: {e}")
            }
        }
    }
}
