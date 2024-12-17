use crate::{is_valid_char, lookup_char, lookup_u64, MAX_CHARS};

const BITS_PER_CHAR: u32 = 6;
const CHAR_MASK: u64 = (1 << BITS_PER_CHAR) - 1;
const MAX_ENCODED_LENGTH: usize = 10; // Maximum safe length for u64 (64 bits / 6 bits per char = 10 chars)

#[inline(always)]
pub fn str_to_int64(s: &str) -> Option<u64> {
    if s.is_empty() {
        return Some(0);
    }

    if s.len() > MAX_ENCODED_LENGTH {
        return None;
    }

    let bytes = s.as_bytes();

    // Validate all characters
    if !bytes.iter().all(|&x| is_valid_char(x)) {
        return None;
    }

    // Encode using unrolled loop for better performance
    let mut result: u64 = 0;
    let len = bytes.len();

    // Process 4 characters at a time from left to right
    let mut i = 0;
    while i + 4 <= len {
        result = (result << (BITS_PER_CHAR * 4))
            | ((lookup_u64(bytes[i]) << (BITS_PER_CHAR * 3))
                | (lookup_u64(bytes[i + 1]) << (BITS_PER_CHAR * 2))
                | (lookup_u64(bytes[i + 2]) << BITS_PER_CHAR)
                | lookup_u64(bytes[i + 3]));
        i += 4;
    }

    // Handle remaining bytes
    while i < len {
        result = (result << BITS_PER_CHAR) | lookup_u64(bytes[i]);
        i += 1;
    }

    Some(result)
}

#[inline(always)]
pub fn int64_to_str(n: u64) -> Option<String> {
    if n == 0 {
        return Some(String::new());
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

    // Process 4 characters at a time from right to left
    let mut i = char_count;
    while i >= 4 {
        i -= 4;
        let chunk = value & ((1 << (BITS_PER_CHAR * 4)) - 1);
        value >>= BITS_PER_CHAR * 4;

        let c0 = chunk & CHAR_MASK;
        let c1 = (chunk >> BITS_PER_CHAR) & CHAR_MASK;
        let c2 = (chunk >> (BITS_PER_CHAR * 2)) & CHAR_MASK;
        let c3 = (chunk >> (BITS_PER_CHAR * 3)) & CHAR_MASK;

        if c0 >= MAX_CHARS || c1 >= MAX_CHARS || c2 >= MAX_CHARS || c3 >= MAX_CHARS {
            return None;
        }

        bytes[i] = lookup_char(c3) as u8;
        bytes[i + 1] = lookup_char(c2) as u8;
        bytes[i + 2] = lookup_char(c1) as u8;
        bytes[i + 3] = lookup_char(c0) as u8;
    }

    // Handle remaining characters
    while i > 0 {
        i -= 1;
        let c = value & CHAR_MASK;
        if c >= MAX_CHARS {
            return None;
        }
        bytes[i] = lookup_char(c) as u8;
        value >>= BITS_PER_CHAR;
    }

    // Convert bytes to string (safe because we only used valid ASCII chars)
    unsafe { Some(String::from_utf8_unchecked(bytes)) }
}
