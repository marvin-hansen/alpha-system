use std::fmt::{Display, Formatter};

use crate::prelude::{Encoding, ProtocolType};

/// An Endpoint represents a single endpoint of a service.
///
/// # Fields
///
/// * `name`: The name of the endpoint.
/// * `version`: The version of the endpoint.
/// * `description`: A description of the endpoint.
/// * `uri`: The Uniform Resource Identifier (URI) of the endpoint.
/// * `port`: The port number of the endpoint.
/// * `protocol`: The protocol type of the endpoint.
/// * `encoding`: The encoding type of the endpoint.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Endpoint {
    name: String,
    version: u8,
    description: String,
    uri: String,
    port: u16,
    protocol: ProtocolType,
    encoding: Encoding,
}

impl Endpoint {
    /// Creates a new Endpoint with the given fields.
    pub fn new(
        name: String,
        version: u8,
        description: String,
        uri: String,
        port: u16,
        protocol: ProtocolType,
        encoding: Encoding,
    ) -> Self {
        Self {
            name,
            version,
            description,
            uri,
            port,
            protocol,
            encoding,
        }
    }

    /// Returns the name of the endpoint.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the version of the endpoint.
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Returns the description of the endpoint.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the URI of the endpoint.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the port number of the endpoint.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the protocol type of the endpoint.
    pub fn protocol(&self) -> &ProtocolType {
        &self.protocol
    }

    /// Returns the encoding type of the endpoint.
    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "name: {},  version: {},  port: {},  description: {},  uri: {},  protocol: {},  encoding: {}",
               self.name, self.version, self.port, self.description, self.uri, self.protocol, self.encoding
        )
    }
}
