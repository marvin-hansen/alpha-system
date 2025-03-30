/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::ExchangeOrderID;
use encoding_utils::{DecodingError, EncodingError, decode_pair_64_to_str, encode_str_to_pair_u64};

/// Extension trait for encoding and decoding a `ExchangeOrderID` to a pair of 64-bit unsigned integers.
///
/// # Methods
///
/// - `encode_to_binary`: Encodes a `ExchangeOrderID` into a pair of 64-bit unsigned integers.
///   Returns a `Result` containing the encoded value or an `EncodingError` if the
///   exchange order ID is too long or contains invalid characters.
/// - `decode_from_binary`: Decodes a pair of 64-bit unsigned integers back into a `ExchangeOrderID`.
///   Returns a `Result` containing the decoded `ExchangeOrderID` or a `DecodingError` if the
///   input integer cannot be decoded.
pub trait ExchangeOrderIdExtension {
    /// Encodes a `ExchangeOrderID` into a pair of 64-bit unsigned integers.
    ///
    /// Returns a `Result` containing the encoded value or an `EncodingError` if the
    /// exchange order ID is too long or contains invalid characters.
    fn encode_to_binary(self) -> Result<(u64, u64), EncodingError>;

    /// Decodes a pair of 64-bit unsigned integers back into a `ExchangeOrderID`.
    ///
    /// Returns a `Result` containing the decoded `ExchangeOrderID` or a `DecodingError` if the
    /// input integer cannot be decoded.
    ///
    fn decode_from_binary(bin: (u64, u64)) -> Result<ExchangeOrderID, DecodingError>;
}

impl ExchangeOrderIdExtension for ExchangeOrderID {
    fn encode_to_binary(self) -> Result<(u64, u64), EncodingError> {
        encode_str_to_pair_u64(self.exchange_order_id())
    }
    fn decode_from_binary(bin: (u64, u64)) -> Result<ExchangeOrderID, DecodingError> {
        let decoded = decode_pair_64_to_str(bin)?;
        if decoded.is_empty() {
            return Err(DecodingError::from("Exchange Order ID is empty"));
        }
        if decoded.len() > 20 {
            return Err(DecodingError::from("Exchange Order ID is too long"));
        }
        Ok(ExchangeOrderID::from(decoded))
    }
}
