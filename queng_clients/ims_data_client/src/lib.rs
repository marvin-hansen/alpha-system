use common::prelude::HostEndpoint;
use proto_bindings::proto::ims_data_service_client::ImsDataServiceClient;
use std::fmt::Error;
use tonic::transport::{Channel, Uri};

pub mod error;
mod utils_proto;
mod workflow;

pub use error::ImsDataClientError;

#[derive(Debug, Clone)]
pub struct ImsDataClient {
    client: ImsDataServiceClient<Channel>,
}

impl ImsDataClient {
    /// Client for interacting with the IMS data service
    ///
    /// Can be used to start/stop data streams from exchanges. Handles
    /// connecting to the gRPC service and sending requests.
    ///
    /// Use the connect() method to create a new instance connected
    /// to the IMS data service. Then call methods like start_data()
    /// and stop_data() to control data streams.
    ///
    pub async fn new(config: HostEndpoint<'_>) -> Result<Self, Error> {
        // Extract host and port from config
        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:4040"
        let s = format!("http://{}:{}", host, port);

        let uri = s.parse::<Uri>().unwrap_or_else(|_| {
            panic!("\r\n ❌ [ImsDataClient]: Failed to parse server URI: {}", s)
        });

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[ImsDataClient]: Failed to connect to Ims Data Service service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = ImsDataServiceClient::new(channel);

        Ok(Self { client })
    }
}
