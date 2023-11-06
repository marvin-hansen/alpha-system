use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ProtocolType {
    #[default]
    GRPC,
    HTTP,
    UDP,
}

impl Display for ProtocolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolType::GRPC => write!(f, "GRPC"),
            ProtocolType::HTTP => write!(f, "HTTP"),
            ProtocolType::UDP => write!(f, "UDP"),
        }
    }
}
