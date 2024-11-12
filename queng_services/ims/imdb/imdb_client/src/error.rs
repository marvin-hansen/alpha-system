use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct IMDBClientError(pub String);

impl Error for IMDBClientError {}

impl Display for IMDBClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMDBClientError: {}", self.0)
    }
}
