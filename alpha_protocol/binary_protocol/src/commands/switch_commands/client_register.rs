use crate::BytesSerializable;
use crate::{Commandable, REGISTER_CLIENT_CODE, Sizeable, Validatable};
use byte_serialization_macro::BytesSerialization;
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use stream_errors::ValidationError;

const BYTE_SIZE: usize = 2;

/// RegisterClient command for client registration
///
/// Optimized for minimal memory footprint and fast processing
#[derive(BytesSerialization, Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct RegisterClient {
    /// Unique client ID.
    client_id: u16, // 2 bytes
}

impl RegisterClient {
    /// Creates a new RegisterClient command with the given client ID
    #[inline(always)]
    pub fn new(client_id: u16) -> Self {
        Self { client_id }
    }

    /// Returns the client ID
    #[inline(always)]
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
}

impl Commandable for RegisterClient {
    #[inline(always)]
    fn command_code(&self) -> u16 {
        REGISTER_CLIENT_CODE
    }
}

impl Sizeable for RegisterClient {
    #[inline(always)]
    fn byte_size(&self) -> usize {
        BYTE_SIZE
    }
}

impl Validatable<ValidationError> for RegisterClient {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        if self.client_id == 0 {
            return Err(ValidationError::ClientIdMustNotBeNull);
        }

        Ok(())
    }
}

impl Display for RegisterClient {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RegisterClient {{ client_id: {} }}", self.client_id)
    }
}
