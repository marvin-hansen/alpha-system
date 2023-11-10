use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// An Enum representing the encoding format used for network communication.
///
/// # Variants
///
/// * `Protobuf`: The Protobuf encoding format.
/// * `SBE`: The SBE (Simple Binary Encoding) format.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Encoding {
    Binary,
    #[default]
    Protobuf,
    SBE,
}

impl Encoding {
    pub fn from_str(s: &str) -> Option<Encoding> {
        match s {
            "BINARY" => Some(Encoding::Binary),
            "PROTOBUF" => Some(Encoding::Protobuf),
            "SBE" => Some(Encoding::SBE),
            _ => None,
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::Binary => write!(f, "Binary"),
            Encoding::Protobuf => write!(f, "Protobuf"),
            Encoding::SBE => write!(f, "SBE"),
        }
    }
}
