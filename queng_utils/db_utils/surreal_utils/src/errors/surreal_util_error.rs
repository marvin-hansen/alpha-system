use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SurrealUtilError(pub String);

impl SurrealUtilError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<String> for SurrealUtilError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for SurrealUtilError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl Error for SurrealUtilError {}

impl fmt::Display for SurrealUtilError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SurrealUtilError: {}", self.0)
    }
}
