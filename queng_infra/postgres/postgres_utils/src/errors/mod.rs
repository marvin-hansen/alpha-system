use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PostgresUtilError(pub String);

impl PostgresUtilError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<String> for PostgresUtilError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for PostgresUtilError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl Error for PostgresUtilError {}

impl fmt::Display for PostgresUtilError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PostgresUtilError: {}", self.0)
    }
}
