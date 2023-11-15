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
    SMDB,
    CMDB,
    DBGW,
}

impl ServiceID {
    pub fn from_string(n: &str) -> Option<ServiceID> {
        match n {
            "Default" => Some(ServiceID::Default),
            "SMDB" => Some(ServiceID::SMDB),
            "CMDB" => Some(ServiceID::CMDB),
            "DBGW" => Some(ServiceID::DBGW),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceID::Default => write!(f, "Default"),
            ServiceID::SMDB => write!(f, "SMDB"),
            ServiceID::CMDB => write!(f, "CMDB"),
            ServiceID::DBGW => write!(f, "DBGW"),
        }
    }
}
