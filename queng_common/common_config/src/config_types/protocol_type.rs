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
#[repr(i8)]
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

impl ProtocolType {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl From<i16> for ProtocolType {
    fn from(value: i16) -> Self {
        match value {
            0x1_i16 => Self::GRPC,
            0x2_i16 => Self::HTTP,
            0x3_i16 => Self::UDP,
            _ => Self::NullVal,
        }
    }
}

impl From<i8> for ProtocolType {
    fn from(value: i8) -> Self {
        match value {
            0x1_i8 => Self::GRPC,
            0x2_i8 => Self::HTTP,
            0x3_i8 => Self::UDP,
            _ => Self::NullVal,
        }
    }
}

impl From<u8> for ProtocolType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0x1_u8 => Self::GRPC,
            0x2_u8 => Self::HTTP,
            0x3_u8 => Self::UDP,
            _ => Self::NullVal,
        }
    }
}

impl From<i32> for ProtocolType {
    /// All .proto enumeration types convert to the Rust i32 type.
    /// This functions converts a raw i32 byte value back into a `ServiceType`.
    /// Unknown message type results in NullVal
    #[inline]
    fn from(value: i32) -> Self {
        match value {
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
