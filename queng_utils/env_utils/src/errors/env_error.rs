use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EnvironmentError(pub String);

impl EnvironmentError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<String> for EnvironmentError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for EnvironmentError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl Error for EnvironmentError {}

impl fmt::Display for EnvironmentError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvironmentError: {}", self.0)
    }
}
