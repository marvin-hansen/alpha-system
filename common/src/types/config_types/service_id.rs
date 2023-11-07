use std::fmt::{Display, Formatter};

/// An Enum that represents the unique ID of a service.
///
/// # Variants
///
/// * `MEMGRAPH`: The Memgraph service.
/// * `SMDB`: The SMDb service.
/// * `CMDB`: The CMDB service.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ServiceID {
    MEMGRAPH,
    #[default]
    SMDB,
    CMDB,
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceID::MEMGRAPH => write!(f, "MEMGRAPH"),
            ServiceID::SMDB => write!(f, "SMDB"),
            ServiceID::CMDB => write!(f, "CMDB"),
        }
    }
}
