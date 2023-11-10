use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// A ProtocolType represents the protocol type used for communication.
///
/// # Variants
///
/// * `GRPC`: The gRPC protocol.
/// * `HTTP`: The HTTP protocol.
/// * `UDP`: The UDP protocol.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ProtocolType {
    /// The gRPC protocol.
    #[default]
    GRPC,
    /// The HTTP protocol.
    HTTP,
    /// The UDP protocol.
    UDP,
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
        }
    }
}
