use crate::BytesSerializable;
use crate::{Commandable, PING_CODE, Sizeable, Validatable};

use byte_serialization_macro::BytesSerialization;
use rkyv::{Archive, Deserialize, Serialize};
use stream_errors::ValidationError;

const BASE_SIZE: usize = 0; // Empty structs are zero bytes

/// Ping command for latency measurement and connection verification
#[derive(BytesSerialization, Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct Ping {}

impl Default for Ping {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Ping {
    /// Creates a new Ping command
    #[inline(always)]
    pub fn new() -> Ping {
        Ping {}
    }
}

impl Commandable for Ping {
    #[inline(always)]
    fn command_code(&self) -> u16 {
        PING_CODE
    }
}

impl Sizeable for Ping {
    #[inline(always)]
    fn byte_size(&self) -> usize {
        BASE_SIZE
    }
}

impl Validatable<ValidationError> for Ping {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        // There is nothing to validate in an empty struct
        Ok(())
    }
}

impl std::fmt::Display for Ping {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ping")
    }
}
