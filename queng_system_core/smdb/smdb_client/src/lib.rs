use proto_smdb::proto::smdb_service_client::SmdbServiceClient;
use tonic::transport::{Channel, Uri};

mod mock_impl;
mod smdb_client_trait;
mod smdb_error;
mod smdb_impl;

pub use smdb_client_trait::SmdbClientTrait;
pub use smdb_error::*;

#[derive(Debug, Clone)]
pub struct SMDBClient {
    client: SmdbServiceClient<Channel>,
}

impl SMDBClient {
    pub async fn new(host: String, smdb_port: u16) -> Self {
        let s = format!("http://{host}:{smdb_port}");
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("[SMDBProvider]: Failed to parse server URI: {s}"));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n [SMDBProvider]: Failed to connect to SMDB service on: {s} \r\n  \r\n Detail: \r\n"));

        let client = SmdbServiceClient::new(channel);

        Self { client }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SMDBCMockClient {}

impl SMDBCMockClient {
    pub async fn new(_host: String, _smdb_port: u16) -> Self {
        Self {}
    }
}
