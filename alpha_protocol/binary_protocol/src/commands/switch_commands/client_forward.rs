use crate::BytesSerializable;
use crate::{Commandable, FORWARD_MESSAGES_CODE, Sizeable, Validatable};
use bon::Builder;
use byte_serialization_macro::BytesSerialization;
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt::Display;
use stream_errors::ValidationError;

const BASE_SIZE: usize = 4;

pub const MAX_PAYLOAD_SIZE: u32 = 1024 * 1024; // 1 MB

/// A SendMessages command whose payload size is known at compile time.
/// The struct is aligned to 64 bytes so that adjacent messages do not share a cache line.
///
#[derive(BytesSerialization, Builder, Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[repr(C, align(64))]
pub struct ForwardMessage {
    /// Messages to sent.
    payload: Vec<u8>,
    cached_payload_size: u32,
}

impl ForwardMessage {
    #[inline(always)]
    pub fn new(payload: Vec<u8>) -> Self {
        // Pre-calculate byte length of the payload Vec<u8>,
        let cached_payload_size = payload.len() as u32;
        Self {
            payload,
            cached_payload_size,
        }
    }

    #[inline(always)]
    pub fn new_validated(payload: Vec<u8>) -> Result<Self, ValidationError> {
        if !payload.is_empty() {
            eprintln!("Payload is empty");
            return Err(ValidationError::EmptyMessagePayload);
        }

        if payload.len() as u32 > MAX_PAYLOAD_SIZE {
            eprintln!(
                "Payload too large: {} > {}",
                payload.len(),
                MAX_PAYLOAD_SIZE
            );
            return Err(ValidationError::PayloadExceedsMaximumSize);
        }

        let cached_payload_size = payload.len() as u32;

        Ok(Self {
            payload,
            cached_payload_size,
        })
    }
}

impl ForwardMessage {
    #[inline(always)]
    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }

    #[inline(always)]
    pub fn cached_payload_size(&self) -> u32 {
        self.cached_payload_size
    }
}

impl Commandable for ForwardMessage {
    fn command_code(&self) -> u16 {
        FORWARD_MESSAGES_CODE
    }
}

impl Validatable<ValidationError> for ForwardMessage {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        if self.payload.is_empty() {
            return Err(ValidationError::EmptyMessagesCount);
        }

        // Use cached payload size for faster validation
        if self.cached_payload_size > MAX_PAYLOAD_SIZE {
            return Err(ValidationError::TooBigMessagePayload(
                self.cached_payload_size,
                MAX_PAYLOAD_SIZE,
            ));
        }

        Ok(())
    }
}

impl Sizeable for ForwardMessage {
    #[inline(always)]
    fn byte_size(&self) -> usize {
        BASE_SIZE + self.cached_payload_size as usize
    }
}

impl Display for ForwardMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForwardMessage")
    }
}
