use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::prelude::{Encoding, HostEndpoint, ProtocolType};

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
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Endpoint<'l> {
    name: &'l str,
    version: u8,
    description: &'l str,
    uri: &'l str,
    port: u16,
    protocol: ProtocolType,
    encoding: Encoding,
}

impl<'l> Endpoint<'l> {
    pub fn new(
        name: &'l str,
        version: u8,
        description: &'l str,
        uri: &'l str,
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
}

impl<'l> Endpoint<'l> {
    pub fn to_memgraph(&self) -> String {
        format!("{{name: '{}', version: {}, description: '{}', uri: '{}', port: {}, protocol: '{}', encoding: '{}'}}",
                self.name, self.version, self.description, self.uri, self.port,
                self.protocol, self.encoding)
    }
}

impl<'l> Endpoint<'l> {
    pub fn name(&self) -> &'l str {
        self.name
    }
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn description(&self) -> &'l str {
        self.description
    }
    pub fn uri(&self) -> &'l str {
        self.uri
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }
}

impl<'l> Endpoint<'l> {
    pub fn host_endpoint(&self) -> HostEndpoint {
        HostEndpoint::new(self.uri(), self.port())
    }
}

impl<'l> Display for Endpoint<'l> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "name: {},  version: {},  port: {},  description: {},  uri: {},  protocol: {},  encoding: {}",
               self.name, self.version, self.port, self.description, self.uri, self.protocol, self.encoding
        )
    }
}
