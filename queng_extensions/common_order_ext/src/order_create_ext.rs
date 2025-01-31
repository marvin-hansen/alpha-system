/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::OrderCreate;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait providing SBE (Simple Binary Encoding) functionality for OrderCreate messages.
///
/// This trait adds encoding and decoding capabilities to the OrderCreate type,
/// allowing conversion between the struct and its binary SBE representation.
pub trait SbeOrderCreateExtension {
    /// Encodes an OrderCreate message into SBE (Simple Binary Encoding) format.
    ///
    /// # Arguments
    /// * `self` - The OrderCreate struct containing creation details
    ///
    /// # Returns
    /// * `Result<(usize, Vec<u8>), SbeEncodeError>` - Either:
    ///   * A tuple containing:
    ///     * `usize`: The size limit of the encoded message
    ///     * `Vec<u8>`: The encoded message buffer as bytes
    ///   * Or `SbeEncodeError` if encoding fails
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes an SBE-encoded OrderCreate message back into its struct form.
    ///
    /// # Arguments
    /// * `data` - A byte slice containing the encoded message
    ///
    /// # Returns
    /// * `Result<OrderCreate, SbeDecodeError>` - Either:
    ///   * The decoded OrderCreate struct containing creation details
    ///   * Or `SbeDecodeError` if decoding fails
    fn decode_from_sbe(data: &[u8]) -> Result<OrderCreate, SbeDecodeError>;
}

impl SbeOrderCreateExtension for OrderCreate {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_order::encode_order_create_message(self)
    }

    fn decode_from_sbe(data: &[u8]) -> Result<OrderCreate, SbeDecodeError> {
        sbe_messages_order::decode_order_create_message(data)
    }
}
