use std::error::Error;
use std::fmt;
use tonic::transport::{Channel, Uri};


use common::prelude::{HostEndpoint};
use proto::binding::smdb_service_client::SmdbServiceClient;

mod prv_smdb;

#[derive(Clone)]
pub struct SMDBProvider {
    client: SmdbServiceClient<Channel>,
}

impl SMDBProvider {
    pub async fn new(smdb_host: String, smdb_port:u16) -> Self {
        let s = format!("http://{}:{}", smdb_host, smdb_port);
        let uri = s.parse::<Uri>()
            .expect(format!("SMDBProvider: Failed to parse server URI: {}", s).as_str());

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .expect(format!("\r\n [SMDBProvider]: Failed to connect to SMDB service on: {} \r\n  \r\n Detail: \r\n", s).as_str());

        let client = SmdbServiceClient::new(channel);

        Self { client }
    }

    /// from_host_endpoint creates a new SMDBProvider from a host endpoint.
    pub async fn from_host_endpoint(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri().to_string();

        Self::new(host, port).await
    }
}

#[derive(Debug)]
pub struct SMDBError(pub String);

impl Error for SMDBError {}

impl fmt::Display for SMDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMDBError: {}", self.0)
    }
}

