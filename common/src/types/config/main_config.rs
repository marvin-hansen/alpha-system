use std::fmt::{Display, Formatter};

use crate::types::config::protocol::Protocol;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MainConfig {
    id: String,
    name: String,
    port: u16,
    protocol: Protocol,
}

impl MainConfig {
    pub fn new(id: String, name: String, port: u16, protocol: Protocol) -> Self {
        Self {
            id,
            name,
            port,
            protocol,
        }
    }
}

impl MainConfig {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn protocol(&self) -> &Protocol {
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
