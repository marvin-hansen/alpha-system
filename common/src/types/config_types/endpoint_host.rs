use std::fmt::{Display, Formatter};

/// Struct that represents a host endpoint.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct HostEndpoint {
    /// Host URI.
    host_uri: String,
    /// Port number.
    port: u16,
}

impl HostEndpoint {
    /// Creates a new `HostEndpoint` instance.
    ///
    /// # Arguments
    ///
    /// * `host_uri` - Host URI.
    /// * `port` - Port number.
    pub fn new(host_uri: String, port: u16) -> Self {
        Self { host_uri, port }
    }

    /// Returns the host URI.
    pub fn host_uri(&self) -> &str {
        &self.host_uri
    }

    /// Returns the port number.
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Display for HostEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "host_uri: {},  port: {}", self.host_uri, self.port)
    }
}