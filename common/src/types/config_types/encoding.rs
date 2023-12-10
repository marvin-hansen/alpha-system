use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// An Enum representing the encoding format used for network communication.
///
/// # Variants
///
/// * `Protobuf`: The Protobuf encoding format.
/// * `SBE`: The SBE (Simple Binary Encoding) format.

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum Encoding {
    #[default]
    NullVal = 0_u8,
    Binary = 1_u8,
    Protobuf = 2_u8,
    SBE = 3_u8,
}

impl From<i32> for Encoding {
    /// All .proto enumeration types convert to the Rust i32 type.
    /// Converts a raw byte value into a `ServiceType`.
    /// Unknown message type results in NullVal
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x1_i32 => Self::Binary,
            0x2_i32 => Self::Protobuf,
            0x3_i32 => Self::SBE,
            _ => Self::NullVal,
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::NullVal => write!(f, "NullVal"),
            Encoding::Binary => write!(f, "Binary"),
            Encoding::Protobuf => write!(f, "Protobuf"),
            Encoding::SBE => write!(f, "SBE"),
        }
    }
}
