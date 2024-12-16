use crate::order_cancel_all::{sbe_decoder, sbe_encoder};
use common_order::OrderCancelAll;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait providing SBE (Simple Binary Encoding) functionality for OrderCancelAll messages.
///
/// This trait adds encoding and decoding capabilities to the OrderCancelAll type,
/// allowing conversion between the struct and its binary SBE representation.
///
/// # Example
/// ```
/// use common_exchange::ExchangeID;
/// use common_order::OrderCancelAll;
/// use sbe_messages_order::SbeOrderCancelAllExtension;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create an order cancel all message
/// let order = OrderCancelAll::new(
///     ExchangeID::Binance,
///     1234,
/// );
///
/// // Encode the order to SBE binary format
/// let (size, encoded_data) = order.encode_to_sbe()?;
///
/// // Decode the binary data back to OrderCancelAll
/// let decoded_order = OrderCancelAll::decode_from_sbe(&encoded_data)?;
/// # Ok(())
/// # }
/// ```
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
        sbe_encoder::encode_order_cancel_all_message(self)
    }

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancelAll, SbeDecodeError> {
        sbe_decoder::decode_order_cancel_all_message(buf)
    }
}
