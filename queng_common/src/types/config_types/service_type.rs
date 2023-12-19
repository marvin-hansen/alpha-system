use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A ServiceType represents the type of service.
///
/// # Variants
///
/// * `ENDPOINT`: An endpoint service type.
/// * `CHANNEL`: The channel service type.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ServiceType {
    NullVal = 0x0_u8,
    /// The endpoint service type.
    #[default]
    ENDPOINT = 0x1_u8,
    /// The channel service type.
    CHANNEL = 0x2_u8,
}

impl From<i32> for ServiceType {
    /// Converts a raw byte value into a `ServiceType`.
    /// Unknown message type results in NullVal
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x0_i32 => Self::NullVal,
            0x1_i32 => Self::ENDPOINT,
            0x2_i32 => Self::CHANNEL,
            _ => Self::NullVal,
        }
    }
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::ENDPOINT => write!(f, "ENDPOINT"),
            ServiceType::CHANNEL => write!(f, "CHANNEL"),
            ServiceType::NullVal => write!(f, "NullVal"),
        }
    }
}
