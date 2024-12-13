use alloc::string::String;

pub const MAX_CHARS: u64 = 37;
const CHARSET: &[u8; 37] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[inline]
pub fn str_to_int(s: &str) -> Option<u64> {
    if s.is_empty() {
        return Some(0);
    }
    if s.len() > MAX_CHARS as usize {
        return None;
    }

    // s.

    Some(0)
}

#[inline]
pub fn int_to_str(mut n: u64) -> Option<String> {
    if n == 0 {
        return Some(String::new());
    }

    Some(String::new())
}
