use crate::BytesSerializable;
use crate::{Commandable, HEARTBEAT_CODE, Sizeable, Validatable};
use byte_serialization_macro::BytesSerialization;
use rkyv::{Archive, Deserialize, Serialize};
use stream_errors::ValidationError;

const BASE_SIZE: usize = 0; // Empty structs are zero bytes

/// Heartbeat command for system health checks
#[derive(BytesSerialization, Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct Heartbeat {}

impl Heartbeat {
    /// Creates a new Heartbeat command
    #[inline(always)]
    pub fn new() -> Heartbeat {
        Heartbeat {}
    }
}

impl Default for Heartbeat {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Commandable for Heartbeat {
    #[inline(always)]
    fn command_code(&self) -> u16 {
        HEARTBEAT_CODE
    }
}

impl Sizeable for Heartbeat {
    #[inline(always)]
    fn byte_size(&self) -> usize {
        BASE_SIZE
    }
}

impl Validatable<ValidationError> for Heartbeat {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        // There is nothing to validate in an empty struct
        Ok(())
    }
}

impl std::fmt::Display for Heartbeat {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Heartbeat")
    }
}
