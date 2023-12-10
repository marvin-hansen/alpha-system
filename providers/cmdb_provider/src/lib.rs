use std::error::Error;
use std::fmt;
use tonic::transport::Channel;
use proto::binding::smdb_service_client::SmdbServiceClient;

#[derive(Debug, Clone)]
pub struct CMDBProvider {
    client: SmdbServiceClient<Channel>,
}

#[derive(Debug)]
pub struct CMDBError(pub String);

impl Error for CMDBError {}

impl fmt::Display for CMDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CMDBError: {}", self.0)
    }
}