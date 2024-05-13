use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct FileServerError(pub String);

impl From<&str> for FileServerError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for FileServerError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for FileServerError {}

impl fmt::Display for FileServerError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileServerError: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct DownloadError(pub String);

impl From<&str> for DownloadError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for DownloadError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DownloadError: {}", self.0)
    }
}

impl Error for DownloadError {}
