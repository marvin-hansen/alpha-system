use crate::order_cancel::{sbe_decoder, sbe_encoder};
use common_order::OrderCancel;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait providing SBE (Simple Binary Encoding) functionality for OrderCancel messages.
///
/// This trait adds encoding and decoding capabilities to the OrderCancel type,
/// allowing conversion between the struct and its binary SBE representation.
///
/// # Example
/// ```
/// use common_exchange::ExchangeID;
/// use common_order::OrderCancel;
/// use sbe_messages_order::SbeOrderCancelExtension;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create an order cancel message
/// let order = OrderCancel::new(
///     ExchangeID::Binance,
///     1234,
///     "ord123".to_string(),     // client_order_id (max 14 chars)
///     "exch456".to_string(),    // exchange_order_id (max 20 chars)
/// );
///
/// // Encode the order to SBE binary format
/// let (size, encoded_data) = order.encode_to_sbe()?;
///
/// // Decode the binary data back to OrderCancel
/// let decoded_order = OrderCancel::decode_from_sbe(&encoded_data)?;
/// # Ok(())
/// # }
/// ```
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
        sbe_encoder::encode_order_cancel_message(self)
    }

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancel, SbeDecodeError> {
        sbe_decoder::decode_order_cancel_message(buf)
    }
}
