use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EnvironmentSetupError(pub String);

impl EnvironmentSetupError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<String> for EnvironmentSetupError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for EnvironmentSetupError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl Error for EnvironmentSetupError {}

impl fmt::Display for EnvironmentSetupError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvironmentSetupError: {}", self.0)
    }
}
