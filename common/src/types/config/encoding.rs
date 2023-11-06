use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Encoding {
    #[default]
    Protobuf,
    SBE,
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::Protobuf => write!(f, "Protobuf"),
            Encoding::SBE => write!(f, "SBE"),
        }
    }
}
