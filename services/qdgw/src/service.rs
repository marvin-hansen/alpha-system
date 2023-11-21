use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;
use tarpc::context::Context;

#[derive(Debug, Serialize, Deserialize)]
pub struct QDGatewayError(pub String);

impl StdError for QDGatewayError {}

impl fmt::Display for QDGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}

/// Service definition. Client and server are generated from this trait.
#[tarpc::service]
pub trait QDGateway {
    async fn start_streaming_data(id: String) -> Result<bool, QDGatewayError>;
}

// This is the type that implements the generated World trait.
// It is the business logic and is used to start the server.
#[derive(Clone)]
pub struct QDGatewayServer {}

impl QDGatewayServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for QDGatewayServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tarpc::server]
impl QDGateway for QDGatewayServer {
    async fn start_streaming_data(self, _: Context, _id: String) -> Result<bool, QDGatewayError> {
        Ok(false)
    }
}
