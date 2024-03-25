use std::error::Error;
use std::fmt;
use tonic::transport::{Channel, Uri};

use proto_bindings::proto::smdb_service_client::SmdbServiceClient;

mod prv_smdb;

#[derive(Debug, Clone)]
pub struct SMDBProvider {
    client: SmdbServiceClient<Channel>,
}

impl SMDBProvider {
    pub async fn new(host: String, smdb_port: u16) -> Self {
        let s = format!("http://{}:{}", host, smdb_port);
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
