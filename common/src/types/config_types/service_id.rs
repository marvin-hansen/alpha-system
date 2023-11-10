use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// An Enum that represents the unique ID of a service.
///
/// # Variants
///
/// * `MEMGRAPH`: The Memgraph service.
/// * `SMDB`: The SMDb service.
/// * `CMDB`: The CMDB service.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ServiceID {
    #[default]
    Default,
    MEMGRAPH,
    SMDB,
    CMDB,
}

impl ServiceID {
    pub fn from_string(n: &str) -> Option<ServiceID> {
        match n {
            "Default" => Some(ServiceID::Default),
            "MEMGRAPH" => Some(ServiceID::MEMGRAPH),
            "SMDB" => Some(ServiceID::SMDB),
            "CMDB" => Some(ServiceID::CMDB),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceID::Default => write!(f, "Default"),
            ServiceID::MEMGRAPH => write!(f, "MEMGRAPH"),
            ServiceID::SMDB => write!(f, "SMDB"),
            ServiceID::CMDB => write!(f, "CMDB"),
        }
    }
}
