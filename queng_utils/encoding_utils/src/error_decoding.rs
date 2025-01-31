/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::fmt;

/// Represents errors that can occur during binary decoding operations.
///
/// This error type is used when converting binary representations back to strings
/// fails due to invalid encoded values, format issues, or other decoding-related problems.
#[derive(Debug, Clone)]
pub struct BinaryDecodingError {
    message: String,
}

impl BinaryDecodingError {
    /// Creates a new BinaryDecodingError with the specified error message.
    ///
    /// # Arguments
    /// * `message` - A description of what caused the decoding error
    pub fn new(message: String) -> Self {
        BinaryDecodingError { message }
    }
}

impl fmt::Display for BinaryDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Binary decoding error: {}", self.message)
    }
}

impl Error for BinaryDecodingError {}

impl From<&str> for BinaryDecodingError {
    fn from(message: &str) -> Self {
        BinaryDecodingError::new(message.to_string())
    }
}

impl From<String> for BinaryDecodingError {
    fn from(message: String) -> Self {
        BinaryDecodingError::new(message)
    }
}
