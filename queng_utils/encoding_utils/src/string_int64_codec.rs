use crate::error_decoding::BinaryDecodingError;
use crate::error_encoding::BinaryEncodingError;
use crate::lookup_tables::{lookup_char, lookup_u64, validate_char};

const BITS_PER_CHAR: u32 = 6;
const CHAR_MASK: u64 = (1 << BITS_PER_CHAR) - 1;
const MAX_ENCODED_LENGTH: usize = 10; // Maximum safe length for u64 (64 bits / 6 bits per char = 10 chars)

/// Encodes a string into a 64-bit unsigned integer.
///
/// # Valid Characters
/// The input string can only contain the following characters:
/// - Uppercase letters (A-Z)
/// - Lowercase letters (a-z)
/// - Digits (0-9)
/// - Underscore (_)
///
/// # Arguments
/// * `s` - The string to encode. Must be at most 10 characters long and contain only valid characters.
///
/// # Returns
/// * `Ok(u64)` - The encoded 64-bit unsigned integer
/// * `Err(BinaryEncodingError)` - If the input string is too long or contains invalid characters
///
/// # Examples
/// ```
/// use encoding_utils::encode_str_to_int64;
///
/// let encoded = encode_str_to_int64("ABC123").unwrap();
/// assert!(encode_str_to_int64("ABC#123").is_err()); // Invalid character '#'
/// assert!(encode_str_to_int64("ABCDEFGHIJK").is_err()); // Too long (>10 chars)
/// ```
#[inline(always)]
pub fn encode_str_to_int64(s: &str) -> Result<u64, BinaryEncodingError> {
    if s.is_empty() {
        return Ok(0);
    }

    if s.len() > MAX_ENCODED_LENGTH {
        return Err(BinaryEncodingError::new(format!(
            "String length {} exceeds maximum allowed length of {}",
            s.len(),
            MAX_ENCODED_LENGTH
        )));
    }

    let bytes = s.as_bytes();
    let mut result: u64 = 0;
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

    // Process 4 characters at a time from left to right
    let mut i = 0;
    while i + 4 <= len {
        let v1 = lookup_u64(bytes[i]);
        let v2 = lookup_u64(bytes[i + 1]);
        let v3 = lookup_u64(bytes[i + 2]);
        let v4 = lookup_u64(bytes[i + 3]);

        result = (result << (BITS_PER_CHAR * 4))
            | ((v1 << (BITS_PER_CHAR * 3))
                | (v2 << (BITS_PER_CHAR * 2))
                | (v3 << BITS_PER_CHAR)
                | v4);
        i += 4;
    }

    // Handle remaining bytes
    while i < len {
        let v = lookup_u64(bytes[i]);
        result = (result << BITS_PER_CHAR) | v;
        i += 1;
    }

    Ok(result)
}

/// Decodes a 64-bit unsigned integer back into a string.
///
/// # Valid Output Characters
/// The decoded string will only contain:
/// - Uppercase letters (A-Z)
/// - Lowercase letters (a-z)
/// - Digits (0-9)
/// - Underscore (_)
///
/// # Arguments
/// * `n` - The 64-bit unsigned integer to decode
///
/// # Returns
/// * `Ok(String)` - The decoded string
/// * `Err(BinaryDecodingError)` - If the input integer contains invalid character mappings
///
/// # Examples
/// ```
/// use encoding_utils::{encode_str_to_int64, decode_int64_to_str};
///
/// // First encode a string to get a valid integer
/// let encoded = encode_str_to_int64("ABC123").unwrap();
/// let decoded = decode_int64_to_str(encoded).unwrap();
/// assert_eq!(decoded, "ABC123");
/// ```
#[inline(always)]
pub fn decode_int64_to_str(n: u64) -> Result<String, BinaryDecodingError> {
    if n == 0 {
        return Ok(String::new());
    }

    // Fast character count using leading zeros
    let char_count = (64 - n.leading_zeros()).div_ceil(BITS_PER_CHAR);
    let char_count = char_count as usize;

    // Pre-allocate exact buffer size
    let mut bytes = Vec::with_capacity(char_count);
    unsafe {
        bytes.set_len(char_count);
    }

    // Use bit manipulation for faster decoding
    let mut value = n;
    let mut i = char_count;

    // Process 4 characters at a time from right to left
    while i >= 4 {
        i -= 4;
        let chunk = value & ((1 << (BITS_PER_CHAR * 4)) - 1);
        value >>= BITS_PER_CHAR * 4;

        // Extract characters using bit operations
        let c0 = chunk & CHAR_MASK;
        let c1 = (chunk >> BITS_PER_CHAR) & CHAR_MASK;
        let c2 = (chunk >> (BITS_PER_CHAR * 2)) & CHAR_MASK;
        let c3 = (chunk >> (BITS_PER_CHAR * 3)) & CHAR_MASK;

        // Convert to characters (lookup table ensures valid characters)
        bytes[i] = lookup_char(c3) as u8;
        bytes[i + 1] = lookup_char(c2) as u8;
        bytes[i + 2] = lookup_char(c1) as u8;
        bytes[i + 3] = lookup_char(c0) as u8;
    }

    // Handle remaining characters (1-3)
    let mut shift = 0;
    while i > 0 {
        i -= 1;
        let c = (value >> shift) & CHAR_MASK;
        bytes[i] = lookup_char(c) as u8;
        shift += BITS_PER_CHAR;
    }

    // Convert bytes to string (safe because lookup table ensures valid ASCII chars)
    Ok(unsafe { String::from_utf8_unchecked(bytes) })
}
