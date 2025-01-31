/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct IMDBClientError(pub String);

impl Error for IMDBClientError {}

impl From<String> for IMDBClientError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for IMDBClientError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for IMDBClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMDBClientError: {}", self.0)
    }
}
