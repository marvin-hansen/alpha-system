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
