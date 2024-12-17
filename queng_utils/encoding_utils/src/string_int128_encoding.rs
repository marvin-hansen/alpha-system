use crate::error_decoding::BinaryDecodingError;
use crate::error_encoding::BinaryEncodingError;
use crate::string_int128_lookup::{lookup_char, lookup_u64};

const MAX_LENGTH_U128: usize = 20; // Maximum length for u128 in base64
const BITS_PER_CHAR_U128: u32 = 6; // 64 possible values = 6 bits
const CHAR_MASK_U128: u128 = (1 << BITS_PER_CHAR_U128) - 1;

/// Encodes a string into a `u128` integer.
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

    let mut result: u128 = 0;
    let mut i = 0;

    // Process 4 characters at a time
    while i + 4 <= len {
        // Validate and decode each character
        for j in 0..4 {
            let c = bytes[i + j];
            if !((c >= b'A' && c <= b'Z')
                || (c >= b'a' && c <= b'z')
                || (c >= b'0' && c <= b'9')
                || c == b'_')
            {
                return Err(BinaryEncodingError::from(format!(
                    "Invalid character at position {}: {}",
                    i + j,
                    c as char
                )));
            }
        }

        let chunk = ((lookup_u64(bytes[i]) << (BITS_PER_CHAR_U128 * 3))
            | (lookup_u64(bytes[i + 1]) << (BITS_PER_CHAR_U128 * 2))
            | (lookup_u64(bytes[i + 2]) << BITS_PER_CHAR_U128)
            | lookup_u64(bytes[i + 3])) as u128;

        result = (result << (BITS_PER_CHAR_U128 * 4)) | chunk;
        i += 4;
    }

    // Handle remaining characters
    while i < len {
        let c = bytes[i];
        // Validate character
        if !((c >= b'A' && c <= b'Z')
            || (c >= b'a' && c <= b'z')
            || (c >= b'0' && c <= b'9')
            || c == b'_')
        {
            return Err(BinaryEncodingError::from(format!(
                "Invalid character at position {}: {}",
                i, c as char
            )));
        }
        result = (result << BITS_PER_CHAR_U128) | lookup_u64(c) as u128;
        i += 1;
    }

    Ok(result)
}

/// Decodes a `u128` integer back into a string.
#[inline(always)]
pub fn decode_int128_to_str(value: u128) -> Result<String, BinaryDecodingError> {
    if value == 0 {
        return Ok(String::new());
    }

    // Pre-allocate exact space needed
    let char_count = (128 - value.leading_zeros()).div_ceil(BITS_PER_CHAR_U128) as usize;
    let mut chars = Vec::with_capacity(char_count);
    let mut current = value;

    // Process 4 characters at a time
    while current >= (1 << (BITS_PER_CHAR_U128 * 4)) {
        let chunk = current & ((1 << (BITS_PER_CHAR_U128 * 4)) - 1);
        current >>= BITS_PER_CHAR_U128 * 4;

        let c0 = lookup_char((chunk & CHAR_MASK_U128) as u64);
        let c1 = lookup_char(((chunk >> BITS_PER_CHAR_U128) & CHAR_MASK_U128) as u64);
        let c2 = lookup_char(((chunk >> (BITS_PER_CHAR_U128 * 2)) & CHAR_MASK_U128) as u64);
        let c3 = lookup_char(((chunk >> (BITS_PER_CHAR_U128 * 3)) & CHAR_MASK_U128) as u64);

        chars.push(c0 as u8);
        chars.push(c1 as u8);
        chars.push(c2 as u8);
        chars.push(c3 as u8);
    }

    // Handle remaining characters
    while current > 0 {
        let c = lookup_char((current & CHAR_MASK_U128) as u64);
        chars.push(c as u8);
        current >>= BITS_PER_CHAR_U128;
    }

    // Reverse the characters since we processed them in reverse order
    chars.reverse();

    // Add all characters to output
    unsafe { Ok(String::from_utf8_unchecked(chars)) }
}
