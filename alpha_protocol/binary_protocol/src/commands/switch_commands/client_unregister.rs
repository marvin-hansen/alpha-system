use crate::BytesSerializable;
use crate::{Commandable, Sizeable, UNREGISTER_CLIENT_CODE, Validatable};
use byte_serialization_macro::BytesSerialization;
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use stream_errors::ValidationError;

const BYTE_SIZE: usize = 2;

/// UnRegisterClient command for client de-registration
///
/// Optimized for minimal memory footprint and fast processing
#[derive(BytesSerialization, Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct UnRegisterClient {
    /// Unique client ID.
    client_id: u16, // 2 bytes
}

impl UnRegisterClient {
    /// Creates a new UnRegisterClient command with the given client ID
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

impl Commandable for UnRegisterClient {
    #[inline(always)]
    fn command_code(&self) -> u16 {
        UNREGISTER_CLIENT_CODE
    }
}

impl Sizeable for UnRegisterClient {
    #[inline(always)]
    fn byte_size(&self) -> usize {
        BYTE_SIZE
    }
}

impl Validatable<ValidationError> for UnRegisterClient {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        if self.client_id == 0 {
            return Err(ValidationError::ClientIdMustNotBeNull);
        }

        Ok(())
    }
}

impl Display for UnRegisterClient {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnRegisterClient {{ client_id: {} }}", self.client_id)
    }
}
