use std::error::Error;

#[derive(Debug, Clone)]
pub struct BinaryEncodingError(pub String);

impl BinaryEncodingError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for BinaryEncodingError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for BinaryEncodingError {
    fn from(value: String) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for BinaryEncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BinaryEncodingError: {}", self.0)
    }
}

impl Error for BinaryEncodingError {}
