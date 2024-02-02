use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DBGatewayError(pub String);

impl Error for DBGatewayError {}

impl fmt::Display for DBGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}
