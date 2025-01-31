/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ClickHouseUtilError(pub String);

impl ClickHouseUtilError {
    #[must_use]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl From<String> for ClickHouseUtilError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl From<&str> for ClickHouseUtilError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl Error for ClickHouseUtilError {}

impl fmt::Display for ClickHouseUtilError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClickHouseUtilError: {}", self.0)
    }
}
