use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// An Enum that represents the unique ID of a service.
///
/// # Variants
///
/// * `MEMGRAPH`: The Memgraph service.
/// * `SMDB`: The SMDb service.
/// * `CMDB`: The CMDB service.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ServiceID {
    #[default]
    Default,
    MEMGRAPH,
    SMDB,
    CMDB,
}

impl ServiceID {
    pub fn to_uint(&self) -> u8 {
        match self {
            ServiceID::Default => 0,
            ServiceID::MEMGRAPH => 1,
            ServiceID::SMDB => 2,
            ServiceID::CMDB => 3,
        }
    }

    pub fn from_uint(n: u8) -> Option<ServiceID> {
        match n {
            0 => Some(ServiceID::Default),
            1 => Some(ServiceID::MEMGRAPH),
            2 => Some(ServiceID::SMDB),
            3 => Some(ServiceID::CMDB),
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
