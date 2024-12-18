use common_data_bar::TradeBar;
use sbe_messages_data::SbeTradeBar;
use sbe_types::{SbeDecodeError, SbeEncodeError};

/// Extension trait for `TradeBar` that provides SBE encoding and decoding
///
/// Encoding and decoding are done using the `SbeTradeBar` struct from the
/// `sbe_messages_data` crate.
pub trait SbeTradeBarExtension {
    /// Encodes a `TradeBar` message to a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `bar` - `TradeBar` to encode
    ///
    /// # Returns
    ///
    /// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
    ///
    /// # Errors
    ///
    /// Returns Err if encoding fails
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    /// Decodes a `TradeBar` message from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Byte buffer containing encoded `TradeBar` message
    ///
    /// # Returns
    ///
    /// Decoded `TradeBar` on success
    ///
    /// # Errors
    ///
    /// Returns Err if decoding fails
    fn decode_from_sbe(encoded: &Vec<u8>) -> Result<TradeBar, SbeDecodeError>;
}

impl SbeTradeBarExtension for TradeBar {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        SbeTradeBar::encode(self)
    }

    fn decode_from_sbe(encoded: &Vec<u8>) -> Result<TradeBar, SbeDecodeError> {
        SbeTradeBar::decode(encoded)
    }
}
