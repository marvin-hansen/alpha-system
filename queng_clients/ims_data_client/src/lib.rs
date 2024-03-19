use common::prelude::HostEndpoint;
use proto::binding::ims_data_service_client::ImsDataServiceClient;
use std::fmt::Error;
use tonic::transport::{Channel, Uri};

mod error;
mod utils_proto;
mod workflow;

#[derive(Debug, Clone)]
pub struct ImsDataClient {
    client: ImsDataServiceClient<Channel>,
}

impl ImsDataClient {
    pub async fn new(config: HostEndpoint<'_>) -> Result<Self, Error> {
        // Extract host and port from config
        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:7070"
        let s = format!("http://{}:{}", host, port);

        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("\r\n ❌ [SymdbClient]: Failed to parse server URI: {}", s));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[SymdbClient]: Failed to connect to SYMDB service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = ImsDataServiceClient::new(channel);

        Ok(Self { client })
    }
}
