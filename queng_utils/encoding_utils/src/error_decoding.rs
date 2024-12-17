use std::error::Error;

#[derive(Debug, Clone)]
pub struct BinaryDecodingError(pub String);

impl BinaryDecodingError {
    pub fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for BinaryDecodingError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for BinaryDecodingError {
    fn from(value: String) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for BinaryDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BinaryDecodingError: {}", self.0)
    }
}

impl Error for BinaryDecodingError {}
