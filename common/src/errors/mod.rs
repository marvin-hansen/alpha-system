use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InitError(pub String);

impl Error for InitError {}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}

