/// Maximum number of characters that can be encoded (limited by u64 size)
pub const MAX_CHARS: usize = 8;

/// Converts a string slice into a u64 integer representation.
/// The string must be ASCII and no longer than 8 characters.
/// Returns None if the string is too long or contains non-ASCII characters.
#[inline(always)]
pub fn str_to_int(s: &str) -> Option<u64> {
    // Branchless length and ASCII check
    let bytes = s.as_bytes();
    let len = bytes.len();
    let is_valid = (len <= MAX_CHARS) as u8 & s.is_ascii() as u8;

    // Early return if invalid
    if is_valid == 0 {
        return None;
    }

    let mut result: u64 = 0;

    // Process bytes in a branchless way
    // SAFETY: We've verified len <= MAX_CHARS, so this won't overflow
    for i in 0..len {
        result |= (bytes[i] as u64) << (i * 8);
    }

    Some(result)
}

/// A buffer for string conversion that uses atomic operations for thread safety
use core::sync::atomic::{AtomicU64, Ordering};

static BUFFER_STORE: AtomicU64 = AtomicU64::new(0);

/// Find the first null byte using SIMD-like bit manipulation
#[inline(always)]
const fn find_null_byte_position(n: u64) -> usize {
    if n == 0 {
        return 0;
    }

    let mut len = 0;
    let mut value = n;

    // Check each byte until we find a null or reach MAX_CHARS
    while len < MAX_CHARS && (value & 0xFF) != 0 {
        len += 1;
        value >>= 8;
    }

    len
}

/// Converts a u64 integer back into its original string representation.
/// Returns None if the integer doesn't represent a valid encoded string.
#[inline(always)]
pub fn int_to_str(n: u64) -> Option<&'static str> {
    // Special case for empty string
    if n == 0 {
        return Some("");
    }

    // Check ASCII validity using SIMD-like bit manipulation
    if n & 0x8080808080808080u64 != 0 {
        return None;
    }

    // Store the value atomically
    BUFFER_STORE.store(0, Ordering::Release); // Clear buffer first
    BUFFER_STORE.store(n, Ordering::Release); // Then store value

    // Find string length
    let len = find_null_byte_position(n);

    // SAFETY: We've verified the bytes are valid ASCII and found the correct length
    unsafe {
        let bytes = &*((&BUFFER_STORE as *const AtomicU64) as *const [u8; 8]);
        Some(core::str::from_utf8_unchecked(&bytes[..len]))
    }
}
