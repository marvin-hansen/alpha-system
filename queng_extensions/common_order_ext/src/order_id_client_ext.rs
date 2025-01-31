/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::ClientOrderID;
use encoding_utils::{decode_int64_to_str, encode_str_to_int64, DecodingError, EncodingError};

/// Extension trait for encoding and decoding a `ClientOrderID` to a single 64-bit unsigned integer.
///
/// # Methods
///
/// - `encode_to_binary`: Encodes a `ClientOrderID` into a 64-bit unsigned integer.
///   Returns a `Result` containing the encoded value or an `EncodingError` if the
///   client order ID is too long or contains invalid characters.
/// - `decode_from_binary`: Decodes a 64-bit unsigned integer back into a `ClientOrderID`.
///   Returns a `Result` containing the decoded `ClientOrderID` or a `DecodingError` if the
///   input integer cannot be decoded.
pub trait ClientOrderIdExtension {
    /// Encodes a `ClientOrderID` into a 64-bit unsigned integer.
    ///
    /// Returns a `Result` containing the encoded value or an `EncodingError` if the
    /// client order ID is too long or contains invalid characters.
    fn encode_to_binary(self) -> Result<u64, EncodingError>;

    /// Decodes a 64-bit unsigned integer back into a `ClientOrderID`.
    ///
    /// Returns a `Result` containing the decoded `ClientOrderID` or a `DecodingError` if the
    /// input integer cannot be decoded.
    fn decode_from_binary(bin: u64) -> Result<ClientOrderID, DecodingError>;
}

impl ClientOrderIdExtension for ClientOrderID {
    fn encode_to_binary(self) -> Result<u64, EncodingError> {
        encode_str_to_int64(self.client_order_id())
    }

    fn decode_from_binary(bin: u64) -> Result<ClientOrderID, DecodingError> {
        let decoded = decode_int64_to_str(bin)?;
        if decoded.is_empty() {
            return Err(DecodingError::from("Client Order ID is empty"));
        }
        if decoded.len() > 10 {
            return Err(DecodingError::from("Client Order ID is too long"));
        }
        Ok(ClientOrderID::from(decoded))
    }
}
