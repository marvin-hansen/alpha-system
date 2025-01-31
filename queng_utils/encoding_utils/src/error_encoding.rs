/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::fmt;

/// Represents errors that can occur during binary encoding operations.
///
/// This error type is used when converting strings to their binary representations
/// fails due to invalid input characters, length constraints, or other encoding-related issues.
#[derive(Debug, Clone)]
pub struct BinaryEncodingError {
    message: String,
}

impl BinaryEncodingError {
    /// Creates a new BinaryEncodingError with the specified error message.
    ///
    /// # Arguments
    /// * `message` - A description of what caused the encoding error
    pub fn new(message: String) -> Self {
        BinaryEncodingError { message }
    }
}

impl fmt::Display for BinaryEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Binary encoding error: {}", self.message)
    }
}

impl Error for BinaryEncodingError {}

impl From<&str> for BinaryEncodingError {
    fn from(message: &str) -> Self {
        BinaryEncodingError::new(message.to_string())
    }
}

impl From<String> for BinaryEncodingError {
    fn from(message: String) -> Self {
        BinaryEncodingError::new(message)
    }
}
