use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct InitError(pub String);

impl Error for InitError {}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBGatewayError(pub String);

impl Error for DBGatewayError {}

impl fmt::Display for DBGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMDBError(pub String);

impl Error for CMDBError {}

impl fmt::Display for CMDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CMDBError: {}", self.0)
    }
}
