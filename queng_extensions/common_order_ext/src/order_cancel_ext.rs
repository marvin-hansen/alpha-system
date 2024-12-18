use common_order::OrderCancel;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait providing SBE (Simple Binary Encoding) functionality for OrderCancel messages.
///
/// This trait adds encoding and decoding capabilities to the OrderCancel type,
/// allowing conversion between the struct and its binary SBE representation.
///
pub trait SbeOrderCancelExtension {
    /// Encodes the OrderCancel message into its SBE binary format.
    ///
    /// # Returns
    /// * `Result<(usize, Vec<u8>), SbeEncodeError>` - A tuple containing:
    ///   * `usize`: The size limit of the encoded message
    ///   * `Vec<u8>`: The encoded message buffer
    ///   * Or `SbeEncodeError` if encoding fails
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes an SBE binary message into an OrderCancel struct.
    ///
    /// # Arguments
    /// * `buf` - A byte slice containing the SBE-encoded message
    ///
    /// # Returns
    /// * `Result<OrderCancel, SbeDecodeError>` - The decoded OrderCancel struct,
    ///    or SbeDecodeError if decoding fails
    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancel, SbeDecodeError>;
}

impl SbeOrderCancelExtension for OrderCancel {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_messages_order::encode_order_cancel_message(self)
    }

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancel, SbeDecodeError> {
        sbe_messages_order::decode_order_cancel_message(buf)
    }
}
