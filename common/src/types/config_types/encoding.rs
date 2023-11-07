use std::fmt::{Display, Formatter};

/// An Enum representing the encoding format used for a message.
///
/// # Variants
///
/// * `Protobuf`: The Protobuf encoding format.
/// * `SBE`: The SBE (Simple Binary Encoding) format.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
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
