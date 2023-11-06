use std::fmt::{Display, Formatter};

use crate::types::config::encoding::Encoding;
use crate::types::config::protocol::Protocol;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Endpoint {
    endpoint_name: String,
    endpoint_version: u8,
    endpoint_description: String,
    endpoint_uri: String,
    endpoint_port: u16,
    endpoint_protocol: Protocol,
    endpoint_encoding: Encoding,
}

impl Endpoint {
    pub fn new(
        endpoint_name: String,
        endpoint_version: u8,
        endpoint_description: String,
        endpoint_uri: String,
        endpoint_port: u16,
        endpoint_protocol: Protocol,
        endpoint_encoding: Encoding,
    ) -> Self {
        Self {
            endpoint_name,
            endpoint_version,
            endpoint_description,
            endpoint_uri,
            endpoint_port,
            endpoint_protocol,
            endpoint_encoding,
        }
    }
}

impl Endpoint {
    pub fn endpoint_name(&self) -> &str {
        &self.endpoint_name
    }
    pub fn endpoint_version(&self) -> u8 {
        self.endpoint_version
    }
    pub fn endpoint_port(&self) -> u16 {
        self.endpoint_port
    }
    pub fn endpoint_description(&self) -> &str {
        &self.endpoint_description
    }
    pub fn endpoint_uri(&self) -> &str {
        &self.endpoint_uri
    }
    pub fn endpoint_protocol(&self) -> &Protocol {
        &self.endpoint_protocol
    }
    pub fn endpoint_encoding(&self) -> &Encoding {
        &self.endpoint_encoding
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "endpoint_name: {}, endpoint_version: {}, endpoint_port: {}, endpoint_description: {}, endpoint_uri: {}, endpoint_protocol: {}, endpoint_encoding: {}",
               self.endpoint_name, self.endpoint_version, self.endpoint_port, self.endpoint_description, self.endpoint_uri, self.endpoint_protocol, self.endpoint_encoding
        )
    }
}
