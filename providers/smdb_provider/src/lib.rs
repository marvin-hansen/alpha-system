use std::error::Error;
use std::fmt;
use tonic::transport::{Channel, Uri};

use common::prelude::HostEndpoint;
use proto::binding::smdb_service_client::SmdbServiceClient;

mod prv_smdb;

#[derive(Debug, Clone)]
pub struct SMDBProvider {
    client: SmdbServiceClient<Channel>,
}

impl SMDBProvider {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri().to_string();

        let s = format!("http://{}:{}", host, port);
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("[SMDBProvider]: Failed to parse server URI: {}", s));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n [SMDBProvider]: Failed to connect to SMDB service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = SmdbServiceClient::new(channel);

        Self { client }
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
