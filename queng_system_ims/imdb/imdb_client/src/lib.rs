mod imdb_client_trait;
mod imdb_error;
mod imdb_impl;
mod imdb_mock;

pub use imdb_client_trait::*;
pub use imdb_error::*;
use std::fmt::Error;

use common_ims::*;
use enum_dispatch::enum_dispatch;
use proto_imdb::proto::imdb_service_client::ImdbServiceClient;
use tonic::transport::{Channel, Uri};

/// An enum for selecting the type of IMDB client to use. This is useful for testing as it allows us to
/// switch between a real IMDB client and a mock client.
///
/// # Usage
/// ```rust, no_run
/// use imdb_client::*;
///
///async fn run() {
/// let host = "127.0.0.1".to_string();
/// let port = 7070;
/// let real_client: IMDBClientSelector = IMDBClient::new(host.clone(), port).await.unwrap().into();
/// let mock_client: IMDBClientSelector = IMDBCMockClient::new(host.clone(), port).await.unwrap().into();
///}
/// ```
///
// https://crates.io/crates/enum-dispatch
#[enum_dispatch]
pub enum IMDBClientSelector {
    /// The real IMDB client
    IMDBClient,
    /// The mock IMDB client
    IMDBCMockClient,
}

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
        let s = format!("http://{host}:{port}");

        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("\r\n ❌ [IMDBClient]: Failed to parse server URI: {s}"));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[IMDBClient]: Failed to connect to IMDB Server service on: {s} \r\n  \r\n Detail: \r\n"));

        let client = ImdbServiceClient::new(channel);

        Ok(Self { client })
    }
}

#[allow(dead_code)] // Ignore the unused field in the struct
#[derive(Debug, Clone)]
pub struct IMDBCMockClient {
    // This field is not used; however,  without it, the auto code formatter would
    // remove the common_ims import, which then causes the enum_dispatch macro to fail compilation.
    // Thus the mock field in the mock client.
    integration_message_config: IntegrationMessageConfig,
    host: String,
    port: u16,
}

impl IMDBCMockClient {
    pub async fn new(host: String, port: u16) -> Result<Self, Error> {
        Ok(Self {
            integration_message_config: IntegrationMessageConfig::default(),
            host,
            port,
        })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
