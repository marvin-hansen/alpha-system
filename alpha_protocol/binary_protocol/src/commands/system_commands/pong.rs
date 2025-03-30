use crate::{BytesSerializable, PONG_CODE};
use crate::{Commandable, Sizeable, Validatable};

use byte_serialization_macro::BytesSerialization;
use rkyv::{Archive, Deserialize, Serialize};
use stream_errors::ValidationError;

const BASE_SIZE: usize = 0; // Empty structs are zero bytes

/// Pong command for latency measurement response
#[derive(BytesSerialization, Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct Pong {}

impl Default for Pong {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Pong {
    /// Creates a new Pong command
    #[inline(always)]
    pub fn new() -> Pong {
        Pong {}
    }
}

impl Commandable for Pong {
    #[inline(always)]
    fn command_code(&self) -> u16 {
        PONG_CODE
    }
}

impl Sizeable for Pong {
    #[inline(always)]
    fn byte_size(&self) -> usize {
        BASE_SIZE
    }
}

impl Validatable<ValidationError> for Pong {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        // There is nothing to validate in an empty struct
        Ok(())
    }
}

impl std::fmt::Display for Pong {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pong")
    }
}
