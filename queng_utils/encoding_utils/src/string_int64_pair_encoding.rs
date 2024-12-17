use crate::error_decoding::BinaryDecodingError;
use crate::error_encoding::BinaryEncodingError;
use crate::string_int128_lookup::{lookup_char, lookup_u64};

const MAX_LENGTH: usize = 20;
const BITS_PER_CHAR: usize = 6;
const CHARS_PER_U64: usize = 10;
const VALID_BITS_MASK: u64 = (1u64 << (BITS_PER_CHAR * CHARS_PER_U64)) - 1;
const CHAR_MASK: u64 = (1u64 << BITS_PER_CHAR) - 1;

/// Encodes up to 20 alphanumeric characters (plus underscore) into a pair of `u64` integers.
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

/// Decodes a pair of `u64` integers back into the original string.
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

    // Pre-allocate with exact capacity
    let mut result = String::with_capacity(MAX_LENGTH);

    // Decode first u64 (first 10 characters)
    let mut shift = 0;
    while shift < BITS_PER_CHAR * CHARS_PER_U64 {
        let value = (first >> shift) & CHAR_MASK;
        if value == 0 {
            break;
        }
        let c = lookup_char(value);
        if c == '\0' {
            return Err(BinaryDecodingError::from(format!(
                "Invalid encoded value at position {}: {}",
                shift / BITS_PER_CHAR,
                value
            )));
        }
        result.push(c);
        shift += BITS_PER_CHAR;
    }

    // Decode second u64 (next 10 characters)
    shift = 0;
    while shift < BITS_PER_CHAR * CHARS_PER_U64 {
        let value = (second >> shift) & CHAR_MASK;
        if value == 0 {
            break;
        }
        let c = lookup_char(value);
        if c == '\0' {
            return Err(BinaryDecodingError::from(format!(
                "Invalid encoded value at position {}: {}",
                CHARS_PER_U64 + shift / BITS_PER_CHAR,
                value
            )));
        }
        result.push(c);
        shift += BITS_PER_CHAR;
    }

    Ok(result)
}
