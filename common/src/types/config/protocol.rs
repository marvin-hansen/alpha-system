use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Protocol {
    GRPC,
    HTTP,
    SBE,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::GRPC
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::GRPC => write!(f, "GRPC"),
            Protocol::HTTP => write!(f, "HTTP"),
            Protocol::SBE => write!(f, "SBE"),
        }
    }
}
