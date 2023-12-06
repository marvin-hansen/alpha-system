use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};
use proto_binding::dbgw::ProtoEndpoint;

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
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
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
}

impl Endpoint {
    pub fn from_proto(proto: ProtoEndpoint) -> Result<Endpoint, Error> {
        let protocol = ProtocolType::from(proto.protocol);
        let encoding = Encoding::from(proto.encoding);

        Ok(Endpoint {
            name: proto.name.to_string(),
            version: proto.version as u8,
            description: proto.description.to_string(),
            uri: proto.uri.to_string(),
            port: proto.port as u16,
            protocol,
            encoding,
        })
    }

    pub fn to_proto(&self) -> Result<ProtoEndpoint, Error> {
        Ok(ProtoEndpoint {
            name: self.name.clone(),
            version: self.version as u32,
            description: self.description.clone(),
            uri: self.uri.clone(),
            port: self.port as u32,
            protocol: self.protocol as i32,
            encoding: self.encoding as i32,
        })
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
    pub fn protocol(&self) -> ProtocolType {
        self.protocol
    }
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }
}

impl Endpoint {
    pub fn host_endpoint(&self) -> HostEndpoint {
        HostEndpoint::new(self.uri(), self.port())
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
