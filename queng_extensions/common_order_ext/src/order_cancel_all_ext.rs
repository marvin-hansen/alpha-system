use common_order::OrderCancelAll;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait providing SBE (Simple Binary Encoding) functionality for OrderCancelAll messages.
///
/// This trait adds encoding and decoding capabilities to the OrderCancelAll type,
/// allowing conversion between the struct and its binary SBE representation.
///
pub trait SbeOrderCancelAllExtension {
    /// Encodes the OrderCancelAll message into its SBE binary format.
    ///
    /// # Returns
    /// * `Result<(usize, Vec<u8>), SbeEncodeError>` - A tuple containing:
    ///   * `usize`: The size limit of the encoded message
    ///   * `Vec<u8>`: The encoded message buffer
    ///   * Or `SbeEncodeError` if encoding fails
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes an SBE binary message into an OrderCancelAll struct.
    ///
    /// # Arguments
    /// * `buf` - A byte slice containing the SBE-encoded message
    ///
    /// # Returns
    /// * `Result<OrderCancelAll, SbeDecodeError>` - The decoded OrderCancelAll struct,
    ///    or SbeDecodeError if decoding fails
    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancelAll, SbeDecodeError>;
}

impl SbeOrderCancelAllExtension for OrderCancelAll {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_order::encode_order_cancel_all_message(self)
    }

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancelAll, SbeDecodeError> {
        sbe_messages_order::decode_order_cancel_all_message(buf)
    }
}
