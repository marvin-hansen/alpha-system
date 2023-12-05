use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// A ProtocolType represents the protocol type used for communication.
///
/// # Variants
///
/// * `GRPC`: The gRPC protocol.
/// * `HTTP`: The HTTP protocol.
/// * `UDP`: The UDP protocol.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum ProtocolType {
    #[default]
    NullVal = 0,
    /// The gRPC protocol.
    GRPC = 1,
    /// The HTTP protocol.
    HTTP = 2,
    /// The UDP protocol.
    UDP = 3,
}

impl From<i32> for ProtocolType {
    /// All .proto enumeration types convert to the Rust i32 type.
    /// This functions converts a raw i32 byte value back into a `ServiceType`.
    /// Unknown message type results in NullVal
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x1_i32 => Self::GRPC,
            0x2_i32 => Self::HTTP,
            0x3_i32 => Self::UDP,
            _ => Self::NullVal,
        }
    }
}

impl ProtocolType {
    pub fn from_string(s: &str) -> Option<ProtocolType> {
        match s {
            "GRPC" => Some(ProtocolType::GRPC),
            "HTTP" => Some(ProtocolType::HTTP),
            "UDP" => Some(ProtocolType::UDP),
            _ => None,
        }
    }
}

impl Display for ProtocolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolType::GRPC => write!(f, "GRPC"),
            ProtocolType::HTTP => write!(f, "HTTP"),
            ProtocolType::UDP => write!(f, "UDP"),
            ProtocolType::NullVal => write!(f, "NullVal"),
        }
    }
}
