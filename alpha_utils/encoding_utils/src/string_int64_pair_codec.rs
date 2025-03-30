/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::error_decoding::BinaryDecodingError;
use crate::error_encoding::BinaryEncodingError;
use crate::lookup_tables::{lookup_char, lookup_u64, validate_char};

const MAX_LENGTH: usize = 20;
const BITS_PER_CHAR: usize = 6;
const CHARS_PER_U64: usize = 10;
const VALID_BITS_MASK: u64 = (1u64 << (BITS_PER_CHAR * CHARS_PER_U64)) - 1;
const CHAR_MASK: u64 = (1u64 << BITS_PER_CHAR) - 1;

/// Encodes a string into a pair of u64 integers using a space-efficient encoding scheme.
///
/// The encoding supports alphanumeric characters (A-Z, a-z, 0-9) and underscore (_).
/// Each character is encoded into 6 bits, allowing for a maximum of 20 characters
/// (10 characters per u64). This encoding scheme is particularly useful when you need
/// to store strings in fixed-size integer fields while maintaining good performance.
///
/// # Arguments
/// * `input` - The string to encode (max 20 characters)
///
/// # Returns
/// * `Ok((u64, u64))` - A tuple containing the two encoded values
/// * `Err(BinaryEncodingError)` - If the input is too long or contains invalid characters
///
/// # Example
/// ```
/// use encoding_utils::encode_str_to_pair_u64;
///
/// let result = encode_str_to_pair_u64("Hello_World123").unwrap();
/// assert!(result.0 > 0 || result.1 > 0);
/// ```
#[inline(always)]
pub fn encode_str_to_pair_u64(input: &str) -> Result<(u64, u64), BinaryEncodingError> {
    let len = input.len();
    if len == 0 {
        return Ok((0, 0));
    }
    if len > MAX_LENGTH {
        return Err(BinaryEncodingError::from(
            "String too long. Max length is 20",
        ));
    }

    let bytes = input.as_bytes();

    // Validate all characters first
    for (i, &byte) in bytes.iter().enumerate() {
        if !validate_char(byte) {
            return Err(BinaryEncodingError::new(format!(
                "Invalid character at position {}: {}",
                i, byte as char
            )));
        }
    }

    let mut first: u64 = 0;
    let mut second: u64 = 0;

    // Process first u64 (up to 10 chars)
    let first_chunk_len = len.min(CHARS_PER_U64);
    for i in 0..first_chunk_len {
        let c = bytes[i];
        let value = lookup_u64(c);
        if value == 0 {
            return Err(BinaryEncodingError::from(format!(
                "Invalid character at position {}: {}",
                i, c as char
            )));
        }
        first |= value << (i * BITS_PER_CHAR);
    }

    // Process second u64 (remaining chars)
    if len > CHARS_PER_U64 {
        for i in CHARS_PER_U64..len {
            let c = bytes[i];
            let value = lookup_u64(c);
            if value == 0 {
                return Err(BinaryEncodingError::from(format!(
                    "Invalid character at position {}: {}",
                    i, c as char
                )));
            }
            second |= value << ((i - CHARS_PER_U64) * BITS_PER_CHAR);
        }
    }

    Ok((first, second))
}

/// Decodes a pair of u64 integers back into the original string.
///
/// This function reverses the encoding performed by `encode_str_to_pair_u64`.
/// It extracts 6 bits at a time from each u64 value and converts them back
/// to their corresponding characters. The first u64 contains the first 10
/// characters, and the second u64 contains the remaining characters.
///
/// # Arguments
/// * `encoded` - A tuple containing the two u64 values to decode
///
/// # Returns
/// * `Ok(String)` - The decoded string
/// * `Err(BinaryDecodingError)` - If the input contains invalid encoded values
///
/// # Example
/// ```
/// use encoding_utils::{encode_str_to_pair_u64, decode_pair_64_to_str};
///
/// let original = "Hello_World123";
/// let encoded = encode_str_to_pair_u64(original).unwrap();
/// let decoded = decode_pair_64_to_str(encoded).unwrap();
/// assert_eq!(original, decoded);
/// ```
#[inline(always)]
pub fn decode_pair_64_to_str(encoded: (u64, u64)) -> Result<String, BinaryDecodingError> {
    let (first, second) = encoded;

    if first == 0 && second == 0 {
        return Ok(String::new());
    }

    // Quick validation of encoded values
    if (first | second) & !VALID_BITS_MASK != 0 {
        return Err(BinaryDecodingError::from(
            "Invalid encoded value: contains bits outside of valid range",
        ));
    }

    // Pre-allocate buffer with exact size
    let mut bytes = Vec::with_capacity(MAX_LENGTH);
    unsafe {
        bytes.set_len(MAX_LENGTH);
    }
    let mut len = 0;

    // Process first u64 (first 10 characters) in chunks of 4
    let mut value = first;
    let mut pos = 0;

    // Process 4 chars at a time
    while pos + 4 <= CHARS_PER_U64 {
        // Extract 4 chars
        let c0 = value & CHAR_MASK;
        let c1 = (value >> BITS_PER_CHAR) & CHAR_MASK;
        let c2 = (value >> (BITS_PER_CHAR * 2)) & CHAR_MASK;
        let c3 = (value >> (BITS_PER_CHAR * 3)) & CHAR_MASK;
        value >>= BITS_PER_CHAR * 4;

        // Early exit if we hit a zero
        if c0 == 0 {
            break;
        }

        unsafe {
            // SAFETY: We've pre-allocated MAX_LENGTH and pos is always < CHARS_PER_U64
            *bytes.get_unchecked_mut(pos) = lookup_char(c0) as u8;
            if c1 == 0 {
                len = pos + 1;
                break;
            }
            *bytes.get_unchecked_mut(pos + 1) = lookup_char(c1) as u8;
            if c2 == 0 {
                len = pos + 2;
                break;
            }
            *bytes.get_unchecked_mut(pos + 2) = lookup_char(c2) as u8;
            if c3 == 0 {
                len = pos + 3;
                break;
            }
            *bytes.get_unchecked_mut(pos + 3) = lookup_char(c3) as u8;
        }

        pos += 4;
        len = pos;
    }

    // Handle remaining chars in first u64
    while pos < CHARS_PER_U64 {
        let c = value & CHAR_MASK;
        value >>= BITS_PER_CHAR;
        if c == 0 {
            break;
        }
        unsafe {
            // SAFETY: len is always < MAX_LENGTH since we're processing first u64
            *bytes.get_unchecked_mut(pos) = lookup_char(c) as u8;
        }
        pos += 1;
        len = pos;
    }

    // Process second u64 if we have more characters
    if second != 0 && len < MAX_LENGTH {
        value = second;
        pos = 0;

        // Process 4 chars at a time
        while pos + 4 <= CHARS_PER_U64 && len + pos + 4 <= MAX_LENGTH {
            // Extract 4 chars
            let c0 = value & CHAR_MASK;
            let c1 = (value >> BITS_PER_CHAR) & CHAR_MASK;
            let c2 = (value >> (BITS_PER_CHAR * 2)) & CHAR_MASK;
            let c3 = (value >> (BITS_PER_CHAR * 3)) & CHAR_MASK;
            value >>= BITS_PER_CHAR * 4;

            // Early exit if we hit a zero
            if c0 == 0 {
                break;
            }

            unsafe {
                // SAFETY: We've checked len + pos + 4 <= MAX_LENGTH
                *bytes.get_unchecked_mut(len + pos) = lookup_char(c0) as u8;
                if c1 == 0 {
                    len += pos + 1;
                    break;
                }
                *bytes.get_unchecked_mut(len + pos + 1) = lookup_char(c1) as u8;
                if c2 == 0 {
                    len += pos + 2;
                    break;
                }
                *bytes.get_unchecked_mut(len + pos + 2) = lookup_char(c2) as u8;
                if c3 == 0 {
                    len += pos + 3;
                    break;
                }
                *bytes.get_unchecked_mut(len + pos + 3) = lookup_char(c3) as u8;
            }

            pos += 4;
            len += 4;
        }

        // Handle remaining chars in second u64
        while pos < CHARS_PER_U64 && len < MAX_LENGTH {
            let c = value & CHAR_MASK;
            value >>= BITS_PER_CHAR;
            if c == 0 {
                break;
            }
            unsafe {
                // SAFETY: We've checked len < MAX_LENGTH
                *bytes.get_unchecked_mut(len) = lookup_char(c) as u8;
            }
            len += 1;
            pos += 1;
        }
    }

    // Truncate to actual length and convert to string
    unsafe {
        bytes.set_len(len);
        // SAFETY: All bytes are valid ASCII from lookup table
        Ok(String::from_utf8_unchecked(bytes))
    }
}
