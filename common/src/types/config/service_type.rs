use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ServiceType {
    #[default]
    ENDPOINT,
    CHANNEL,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::ENDPOINT => write!(f, "ENDPOINT"),
            ServiceType::CHANNEL => write!(f, "CHANNEL"),
        }
    }
}
