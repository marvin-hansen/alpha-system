use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum ServiceName {
    #[default]
    UNKNOWN,
    MEMGRAPH,
    SMDB,
    CMDB,
}

impl Display for ServiceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceName::UNKNOWN => write!(f, "UNKNOWN"),
            ServiceName::MEMGRAPH => write!(f, "MEMGRAPH"),
            ServiceName::SMDB => write!(f, "SMDB"),
            ServiceName::CMDB => write!(f, "CMDB"),
        }
    }
}
