use std::fmt::{Display, Formatter};

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
