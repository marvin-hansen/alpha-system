use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct ImsDataClientError(pub String);

impl Error for ImsDataClientError {}

impl Display for ImsDataClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImsDataClientError: {}", self.0)
    }
}
