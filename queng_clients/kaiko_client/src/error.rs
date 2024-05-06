use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct KaikoClientError(pub String);

impl KaikoClientError {
    pub fn new(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<&str> for KaikoClientError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for KaikoClientError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for KaikoClientError {}

impl fmt::Display for KaikoClientError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KaikoClientError: {}", self.0)
    }
}
