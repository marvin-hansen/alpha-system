use std::fmt::{Display, Formatter};

use crate::prelude::{ProtocolType, ServiceID};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MainConfig {
    id: ServiceID,
    name: String,
    port: u16,
    protocol: ProtocolType,
}

impl MainConfig {
    pub fn new(id: ServiceID, name: String, port: u16, protocol: ProtocolType) -> Self {
        Self {
            id,
            name,
            port,
            protocol,
        }
    }
}

impl MainConfig {
    pub fn id(&self) -> &ServiceID {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn port(&self) -> u16 {
        self.port
    }
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
