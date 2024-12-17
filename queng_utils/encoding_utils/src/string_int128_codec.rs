use crate::error_decoding::BinaryDecodingError;
use crate::error_encoding::BinaryEncodingError;
use crate::lookup_tables::{lookup_char, lookup_u64, validate_char};

const MAX_LENGTH_U128: usize = 20; // Maximum length for u128 in base64
const BITS_PER_CHAR_U128: u32 = 6; // 64 possible values = 6 bits
const CHAR_MASK_U128: u128 = (1 << BITS_PER_CHAR_U128) - 1;

/// Encodes a string into a u128 integer using a 6-bit character encoding scheme.
///
/// The encoding supports alphanumeric characters (A-Z, a-z, 0-9) and underscore (_).
/// Each character is encoded into 6 bits, allowing for a maximum of 21 characters.
/// The encoding is done in chunks of 4 characters for optimal performance.
///
/// # Arguments
/// * `input` - The string to encode
///
/// # Returns
/// * `Ok(u128)` - The encoded value
/// * `Err(BinaryEncodingError)` - If the input contains invalid characters or is too long
///
/// # Example
/// ```
/// use encoding_utils::encode_str_to_int128;
///
/// let result = encode_str_to_int128("Hello123").unwrap();
/// assert!(result > 0);
/// ```
#[inline(always)]
pub fn encode_str_to_int128(input: &str) -> Result<u128, BinaryEncodingError> {
    if input.is_empty() {
        return Ok(0);
    }

    if input.len() > MAX_LENGTH_U128 {
        return Err(BinaryEncodingError::from(
            "String too long. Max length is 20",
        ));
    }

    let bytes = input.as_bytes();
    let len = bytes.len();

    // Validate all characters first
    for (i, &byte) in bytes.iter().enumerate() {
        if !validate_char(byte) {
            return Err(BinaryEncodingError::new(format!(
                "Invalid character at position {}: {}",
                i, byte as char
            )));
        }
    }

    let mut result: u128 = 0;
    let mut i = 0;

    // Process 4 characters at a time
    while i + 4 <= len {
        let chunk = ((lookup_u64(bytes[i]) << (BITS_PER_CHAR_U128 * 3))
            | (lookup_u64(bytes[i + 1]) << (BITS_PER_CHAR_U128 * 2))
            | (lookup_u64(bytes[i + 2]) << BITS_PER_CHAR_U128)
            | lookup_u64(bytes[i + 3])) as u128;

        result = (result << (BITS_PER_CHAR_U128 * 4)) | chunk;
        i += 4;
    }

    // Handle remaining characters
    while i < len {
        result = (result << BITS_PER_CHAR_U128) | lookup_u64(bytes[i]) as u128;
        i += 1;
    }

    Ok(result)
}

/// Decodes a u128 integer back into its original string representation.
///
/// This function reverses the encoding performed by `encode_str_to_int128`.
/// It extracts 6 bits at a time from the input value and converts them back
/// to their corresponding characters.
///
/// # Arguments
/// * `input` - The u128 value to decode
///
/// # Returns
/// * `Ok(String)` - The decoded string
/// * `Err(BinaryDecodingError)` - If the input contains invalid encoded values
///
/// # Example
/// ```
/// use encoding_utils::{encode_str_to_int128, decode_int128_to_str};
///
/// let original = "Hello123";
/// let encoded = encode_str_to_int128(original).unwrap();
/// let decoded = decode_int128_to_str(encoded).unwrap();
/// assert_eq!(original, decoded);
/// ```
#[inline(always)]
pub fn decode_int128_to_str(input: u128) -> Result<String, BinaryDecodingError> {
    if input == 0 {
        return Ok(String::new());
    }

    // Pre-allocate fixed-size array on stack
    let mut chars = [0u8; MAX_LENGTH_U128];
    let mut len = 0;

    // Calculate total characters based on highest set bit
    let total_bits = 128 - input.leading_zeros();
    let char_count = total_bits.div_ceil(BITS_PER_CHAR_U128);
    let mut remaining_bits = char_count * BITS_PER_CHAR_U128;

    // Process characters in chunks of 4 from most significant to least significant
    while remaining_bits >= BITS_PER_CHAR_U128 * 4 {
        remaining_bits -= BITS_PER_CHAR_U128 * 4;
        let chunk = (input >> remaining_bits) & ((1 << (BITS_PER_CHAR_U128 * 4)) - 1);

        // Extract characters
        let c0 = lookup_char(((chunk >> (BITS_PER_CHAR_U128 * 3)) & CHAR_MASK_U128) as u64) as u8;
        let c1 = lookup_char(((chunk >> (BITS_PER_CHAR_U128 * 2)) & CHAR_MASK_U128) as u64) as u8;
        let c2 = lookup_char(((chunk >> BITS_PER_CHAR_U128) & CHAR_MASK_U128) as u64) as u8;
        let c3 = lookup_char((chunk & CHAR_MASK_U128) as u64) as u8;

        // Validate all characters in chunk
        for (i, &c) in [c0, c1, c2, c3].iter().enumerate() {
            if !validate_char(c) {
                return Err(BinaryDecodingError::new(format!(
                    "Invalid character at position {}: {}",
                    len + i,
                    c as char
                )));
            }
        }

        // Store characters
        chars[len] = c0;
        chars[len + 1] = c1;
        chars[len + 2] = c2;
        chars[len + 3] = c3;
        len += 4;
    }

    // Handle remaining characters
    while remaining_bits > 0 {
        remaining_bits -= BITS_PER_CHAR_U128;
        let c = lookup_char(((input >> remaining_bits) & CHAR_MASK_U128) as u64) as u8;
        if !validate_char(c) {
            return Err(BinaryDecodingError::new(format!(
                "Invalid character at position {}: {}",
                len, c as char
            )));
        }
        chars[len] = c;
        len += 1;
    }

    // Create string from validated characters
    // SAFETY: We know all bytes in chars[..len] are valid ASCII characters
    // as we validated them above
    Ok(unsafe { String::from_utf8_unchecked(chars[..len].to_vec()) })
}
