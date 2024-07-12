mod error;
mod utils_proto;
mod workflow;

use common::prelude::HostEndpoint;
use proto_bindings::proto::mdm_service_client::MdmServiceClient;
use std::fmt::Error;
use tonic::transport::{Channel, Uri};

/// Client for interacting with the SymdbService.
///
/// Wraps a SymdbServiceClient and provides methods to
/// lookup symbols, symbol IDs, and exchange names.
///
#[derive(Debug, Clone)]
pub struct MDMClient {
    client: MdmServiceClient<Channel>,
}

impl MDMClient {
    /// Creates a new SymdbClient instance.
    ///
    /// # Arguments
    ///
    /// * `config: HostEndpoint` - The endpoint configuration of the SYMDB Service gRPC server
    ///
    /// # Returns
    ///
    /// Returns a SymdbClient connected to the given address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    /// use symdb_client::MDMClient;
    /// use common::prelude::HostEndpoint;
    ///
    ///     let config = HostEndpoint::new("127.0.0.1", 7070);
    ///     let client = MDMClient::new(config).await.expect("Failed to create SymdbClient");
    /// }
    /// ```
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

        let client = MdmServiceClient::new(channel);

        Ok(Self { client })
    }
}
