use std::fmt::{Display, Formatter};

use crate::prelude::{Encoding, Protocol};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Endpoint {
    name: String,
    version: u8,
    description: String,
    uri: String,
    port: u16,
    protocol: Protocol,
    encoding: Encoding,
}

impl Endpoint {
    pub fn new(
        name: String,
        version: u8,
        description: String,
        uri: String,
        port: u16,
        protocol: Protocol,
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

impl Endpoint {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn uri(&self) -> &str {
        &self.uri
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }
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
