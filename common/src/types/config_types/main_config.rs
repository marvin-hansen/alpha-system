use std::fmt::{Display, Formatter};

use crate::prelude::{ProtocolType, ServiceID};

/// MainConfig represents the main configuration for the service.
///
/// # Fields
///
/// * `id`: The service ID.
/// * `name`: The service name.
/// * `port`: The port number.
/// * `protocol`: The protocol type.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MainConfig {
    id: ServiceID,
    name: String,
    port: u16,
    protocol: ProtocolType,
}

impl MainConfig {
    /// Creates a new MainConfig with the given fields.
    pub fn new(id: ServiceID, name: String, port: u16, protocol: ProtocolType) -> Self {
        Self {
            id,
            name,
            port,
            protocol,
        }
    }

    /// Returns the service ID.
    pub fn id(&self) -> &ServiceID {
        &self.id
    }

    /// Returns the service name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the port number.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the protocol type.
    pub fn protocol(&self) -> &ProtocolType {
        &self.protocol
    }
}

impl Display for MainConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MainConfig {{ id: {:?}, name: {:?}, port: {}, protocol: {:?} }}",
            self.id, self.name, self.port, self.protocol
        )
    }
}
