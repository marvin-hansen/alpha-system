use std::fmt::{Display, Formatter};

/// A ProtocolType represents the protocol type used for communication.
///
/// # Variants
///
/// * `GRPC`: The gRPC protocol.
/// * `HTTP`: The HTTP protocol.
/// * `UDP`: The UDP protocol.
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

impl Display for ProtocolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolType::GRPC => write!(f, "GRPC"),
            ProtocolType::HTTP => write!(f, "HTTP"),
            ProtocolType::UDP => write!(f, "UDP"),
        }
    }
}
