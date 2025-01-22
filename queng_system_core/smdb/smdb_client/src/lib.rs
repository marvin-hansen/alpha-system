use common_config::*;
use enum_dispatch::enum_dispatch;
use proto_smdb::proto::smdb_service_client::SmdbServiceClient;
use tonic::transport::{Channel, Uri};

mod mock_impl;
mod smdb_client_trait;
mod smdb_error;
mod smdb_impl;

pub use smdb_client_trait::*;
pub use smdb_error::*;

/// A dispatch enum for selecting the type of SMDB client to use. This is useful for testing as it allows us to
/// switch between a real IMDB client and a mock client.
///
/// !!!! NOTE: The SMDBClientSelector is exposed via the client_util crate for context dependent client selection.
/// !!!! Use the client_util crate instead of directly importing this module when you need to select clients dynamically.
///
/// # Usage
/// ```rust, no_run
/// use smdb_client::*;
///
///async fn run() {
/// let host = "127.0.0.1".to_string();
/// let port = 7070;
/// let real_client: SMDBClientSelector = SMDBClient::new(host.clone(), port).await.into();
/// let mock_client: SMDBClientSelector = SMDBCMockClient::new(host.clone(), port).await.into();
///}
/// ```
///
// https://crates.io/crates/enum-dispatch
#[enum_dispatch]
pub enum SMDBClientSelector {
    /// The real SMDB client
    SMDBClient,
    /// The mock SMDB client
    SMDBCMockClient,
}

#[derive(Debug, Clone)]
pub struct SMDBClient {
    client: SmdbServiceClient<Channel>,
}

impl SMDBClient {
    /// Creates a new SMDB client instance by establishing a connection to the specified host and port.
    ///
    /// # Arguments
    /// * `host` - The host address of the SMDB server
    /// * `smdb_port` - The port number of the SMDB server
    ///
    /// # Returns
    /// * `Self` - A new SMDB client instance on success
    ///
    /// # Panics
    /// * If URI parsing fails
    /// * If connection to SMDB server fails
    ///
    pub async fn new(host: String, smdb_port: u16) -> Self {
        let s = format!("http://{host}:{smdb_port}");
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("\r\n ❌[SMDBClient]: Failed to parse server URI: {s}"));

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[SMDBClient]: Failed to connect to SMDB service on: {s} \r\n  \r\n Detail: \r\n"));

        let client = SmdbServiceClient::new(channel);

        Self { client }
    }
}

#[allow(dead_code)] // Ignore the unused field in the struct
#[derive(Debug, Clone)]
pub struct SMDBCMockClient {
    // This field is not used; however,  without it, the auto code formatter would
    // remove the common_config import, which then causes the enum_dispatch macro to fail compilation.
    service_id: ServiceID,
    host: String,
    port: u16,
}

impl SMDBCMockClient {
    /// Creates a new SMDB mock client instance with the specified host and port.
    ///
    /// # Arguments
    /// * `host` - The host address of the SMDB server
    /// * `port` - The port number of the SMDB server
    ///
    /// # Returns
    /// * `Result<Self, Error>` - A new SMDB client instance on success, or an Error on failure
    pub async fn new(host: String, port: u16) -> Self {
        Self {
            service_id: ServiceID::Default,
            host,
            port,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
