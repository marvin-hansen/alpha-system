use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MDDBClientError(pub String);

impl Error for MDDBClientError {}

impl Display for MDDBClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MDDBClientError: {}", self.0)
    }
}
