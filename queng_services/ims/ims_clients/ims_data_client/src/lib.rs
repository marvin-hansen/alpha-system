use common_config::prelude::HostEndpoint;
use std::fmt::Error;
use tonic::transport::Uri;

pub mod error;
mod workflow;

pub use error::ImsDataClientError;

#[derive(Debug, Clone)]
pub struct ImsDataClient {
    uri: Uri,
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

        Ok(Self { uri })
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}
