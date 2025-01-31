/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use encoding_utils::{decode_int64_to_str, encode_str_to_int64};

fn main() {
    // Example 1: Basic encoding and decoding
    let original = "Hello";
    let encoded = encode_str_to_int64(original).expect("Failed to encode");
    let decoded = decode_int64_to_str(encoded).expect("Failed to decode");
    println!(
        "Original: '{}'\nEncoded: {}\nDecoded: '{}'",
        original, encoded, decoded
    );
    assert_eq!(original, decoded);

    // Example 2: Empty string
    let empty = "";
    let encoded_empty = encode_str_to_int64(empty).expect("Failed to encode empty");
    let decoded_empty = decode_int64_to_str(encoded_empty).expect("Failed to decode empty");
    println!(
        "\nEmpty string:\nEncoded: {}\nDecoded: '{}'",
        encoded_empty, decoded_empty
    );
    assert_eq!(empty, decoded_empty);

    // Example 3: Maximum length string
    let max_str = "abcdefgh"; // 8 characters for int64
    let encoded_max = encode_str_to_int64(max_str).expect("Failed to encode max");
    let decoded_max = decode_int64_to_str(encoded_max).expect("Failed to decode max");
    println!(
        "\nMax length string:\nOriginal: '{}'\nEncoded: {}\nDecoded: '{}'",
        max_str, encoded_max, decoded_max
    );
    assert_eq!(max_str, decoded_max);
}
