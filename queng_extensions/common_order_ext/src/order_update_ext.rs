/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::OrderUpdate;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait providing SBE (Simple Binary Encoding) functionality for OrderUpdate messages.
///
/// This trait adds encoding and decoding capabilities to the OrderUpdate type,
/// allowing conversion between the struct and its binary SBE representation.
///
pub trait SbeOrderUpdateExtension {
    /// Encodes this OrderUpdate into its SBE representation.
    ///
    /// # Returns
    /// * `Result<(usize, Vec<u8>), SbeEncodeError>` - A tuple containing:
    ///   * `usize`: The size limit of the encoded message
    ///   * `Vec<u8>`: The encoded message buffer as bytes
    ///   * Or `SbeEncodeError` if encoding fails
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes an SBE-encoded message back into its OrderUpdate struct form.
    ///
    /// # Returns
    /// * `Result<OrderUpdate, SbeDecodeError>` - Either:
    ///   * The decoded OrderUpdate struct containing cancellation details
    ///   * Or `SbeDecodeError` if decoding fails
    fn decode_from_sbe(data: &[u8]) -> Result<OrderUpdate, SbeDecodeError>;
}

impl SbeOrderUpdateExtension for OrderUpdate {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_order::encode_order_update_message(self)
    }

    fn decode_from_sbe(data: &[u8]) -> Result<OrderUpdate, SbeDecodeError> {
        sbe_messages_order::decode_order_update_message(data)
    }
}
