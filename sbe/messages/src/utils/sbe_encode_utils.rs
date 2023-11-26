#[inline]
pub fn encode_string<const N: usize>(str: &str) -> [u8; N] {
    let array: [u8; N] =
        <[u8; N]>::try_from(str.as_bytes()).expect("Failed to convert string to array");
    array
}
