/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SMDBError(pub String);

impl Error for SMDBError {}

impl fmt::Display for SMDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMDBError: {}", self.0)
    }
}
