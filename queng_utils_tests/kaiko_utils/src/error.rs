use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct KaikoUtilError(pub String);

impl KaikoUtilError {
    pub fn new(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<&str> for KaikoUtilError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for KaikoUtilError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for KaikoUtilError {}

impl fmt::Display for KaikoUtilError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KaikoUtilError: {}", self.0)
    }
}
