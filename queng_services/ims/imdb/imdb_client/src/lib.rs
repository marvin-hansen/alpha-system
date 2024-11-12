mod error;
mod imdb_integrations;

use proto_imdb::proto::imdb_service_client::ImdbServiceClient;
use std::fmt::Error;
use tonic::transport::{Channel, Uri};

#[derive(Debug, Clone)]
pub struct IMDBClient {
    client: ImdbServiceClient<Channel>,
}

impl IMDBClient {
    /// Creates a new IMDB client instance by establishing a connection to the specified host and port.
    ///
    /// # Arguments
    /// * `host` - The host address of the IMDB server
    /// * `port` - The port number of the IMDB server
    ///
    /// # Returns
    /// * `Result<Self, Error>` - A new IMDB client instance on success, or an Error on failure
    ///
    /// # Panics
    /// * If URI parsing fails
    /// * If connection to IMDB server fails
    pub async fn new(host: String, port: u16) -> Result<Self, Error> {
        // "http://[::1]:7070"
        let s = format!("http://{}:{}", host, port);

        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("\r\n ❌ [IMDBClient]: Failed to parse server URI: {}", s));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[IMDBClient]: Failed to connect to IMDB Server service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = ImdbServiceClient::new(channel);

        Ok(Self { client })
    }
}
