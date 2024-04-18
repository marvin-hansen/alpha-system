use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TestEnvError(pub String);

impl TestEnvError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<String> for TestEnvError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for TestEnvError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl Error for TestEnvError {}

impl fmt::Display for TestEnvError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
