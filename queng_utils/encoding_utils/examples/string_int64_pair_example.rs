/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use encoding_utils::{decode_pair_64_to_str, encode_str_to_pair_u64};

fn main() {
    // Example 1: Basic encoding and decoding
    let original = "HelloWorld123"; // No spaces, only valid characters
    let (high, low) = encode_str_to_pair_u64(original).expect("Failed to encode");
    let decoded = decode_pair_64_to_str((high, low)).expect("Failed to decode");
    println!(
        "Original: '{}'\nEncoded: ({}, {})\nDecoded: '{}'",
        original, high, low, decoded
    );
    assert_eq!(original, decoded);

    // Example 2: Empty string
    let empty = "";
    let (high_empty, low_empty) = encode_str_to_pair_u64(empty).expect("Failed to encode empty");
    let decoded_empty =
        decode_pair_64_to_str((high_empty, low_empty)).expect("Failed to decode empty");
    println!(
        "\nEmpty string:\nEncoded: ({}, {})\nDecoded: '{}'",
        high_empty, low_empty, decoded_empty
    );
    assert_eq!(empty, decoded_empty);

    // Example 3: Maximum length string (20 characters)
    let max_str = "01234567890123456789"; // 20 characters
    let (high_max, low_max) = encode_str_to_pair_u64(max_str).expect("Failed to encode max");
    let decoded_max = decode_pair_64_to_str((high_max, low_max)).expect("Failed to decode max");
    println!(
        "\nMax length string:\nOriginal: '{}'\nEncoded: ({}, {})\nDecoded: '{}'",
        max_str, high_max, low_max, decoded_max
    );
    assert_eq!(max_str, decoded_max);

    // Example 4: Mixed character types (valid characters only)
    let mixed = "Hello123ABC456DEF789"; // 18 characters
    let (high_mixed, low_mixed) = encode_str_to_pair_u64(mixed).expect("Failed to encode mixed");
    let decoded_mixed =
        decode_pair_64_to_str((high_mixed, low_mixed)).expect("Failed to decode mixed");
    println!(
        "\nMixed characters:\nOriginal: '{}'\nEncoded: ({}, {})\nDecoded: '{}'",
        mixed, high_mixed, low_mixed, decoded_mixed
    );
    assert_eq!(mixed, decoded_mixed);

    // Example 5: Error handling for too long string
    let too_long = "012345678901234567890"; // 21 characters
    match encode_str_to_pair_u64(too_long) {
        Ok(_) => println!("Unexpectedly encoded too long string"),
        Err(e) => println!("\nExpected error for too long string: {}", e),
    }
}
