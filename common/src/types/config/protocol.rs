use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Protocol {
    #[default]
    GRPC,
    HTTP,
    UDP,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::GRPC => write!(f, "GRPC"),
            Protocol::HTTP => write!(f, "HTTP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}
