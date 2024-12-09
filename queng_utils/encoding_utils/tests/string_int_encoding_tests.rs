use encoding_utils::{int_to_str, str_to_int};

/// Test basic encoding and decoding functionality
#[test]
fn test_basic_encoding() {
    let test_cases = ["hello", "world", "rust", "test", "code"];

    for &input in &test_cases {
        let encoded = str_to_int(input).unwrap();
        let decoded = int_to_str(encoded).unwrap();
        assert_eq!(input, decoded, "Failed on input: {}", input);
    }
}

/// Test empty string handling
#[test]
fn test_empty_string() {
    let encoded = str_to_int("").unwrap();
    assert_eq!(0, encoded, "Empty string should encode to 0");
    let decoded = int_to_str(encoded).unwrap();
    assert_eq!("", decoded, "Decoded empty string should be empty");
}

/// Test maximum length stringsf
#[test]
fn test_max_length() {
    let test_cases = ["12345678", "abcdefgh", "ABCDEFGH", "!@#$%^&*"];

    for &input in &test_cases {
        let encoded = str_to_int(input).unwrap();
        let decoded = int_to_str(encoded).unwrap();
        assert_eq!(input, decoded, "Failed on max length input: {}", input);
    }

    // Test strings that are too long
    let too_long_cases = ["123456789", "abcdefghi", "ABCDEFGHI", "!@#$%^&*()"];

    for &input in &too_long_cases {
        assert!(
            str_to_int(input).is_none(),
            "Should reject input: {}",
            input
        );
    }
}

/// Test strings of all possible lengths
#[test]
fn test_all_lengths() {
    // Test with different characters for each length
    let chars = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    for len in 0..=8 {
        let input: String = chars.iter().take(len).collect();
        let encoded = str_to_int(&input).unwrap();
        let decoded = int_to_str(encoded).unwrap();
        assert_eq!(input, decoded, "Failed on length {}", len);
    }
}

/// Test non-ASCII character handling
#[test]
fn test_non_ascii() {
    let test_cases = ["hello❤", "world★", "rust☺", "test→", "code∞"];

    for &input in &test_cases {
        assert!(
            str_to_int(input).is_none(),
            "Should reject non-ASCII: {}",
            input
        );
    }
}

/// Test invalid encoded integers
#[test]
fn test_invalid_encoded() {
    let test_cases = [
        0xFF00_0000_0000_0000,
        0x8000_0000_0000_0000,
        0xFFFF_FFFF_FFFF_FFFF,
    ];

    for &input in &test_cases {
        assert!(
            int_to_str(input).is_none(),
            "Should reject invalid encoded: {:x}",
            input
        );
    }
}

/// Test mixed case strings
#[test]
fn test_mixed_case() {
    let test_cases = ["AbCdEfGh", "tEsT", "RuSt", "HeLLo"];

    for &input in &test_cases {
        let encoded = str_to_int(input).unwrap();
        let decoded = int_to_str(encoded).unwrap();
        assert_eq!(input, decoded, "Failed on mixed case: {}", input);
    }
}

/// Test strings with special characters
#[test]
fn test_special_chars() {
    let test_cases = ["!@#$", "%^&*", "()_+", "-=[]", "{}|\\"];

    for &input in &test_cases {
        let encoded = str_to_int(input).unwrap();
        let decoded = int_to_str(encoded).unwrap();
        assert_eq!(input, decoded, "Failed on special chars: {}", input);
    }
}

/// Test null byte handling
#[test]
fn test_null_bytes() {
    // Test strings with embedded null bytes (should encode up to first null)
    let encoded = str_to_int("ab\0cd").unwrap();
    let decoded = int_to_str(encoded).unwrap();
    assert_eq!("ab", decoded, "Should handle null bytes correctly");
}

/// Test edge cases for integer encoding
#[test]
fn test_integer_edges() {
    let test_cases = [
        0u64,
        1u64,
        u64::MAX >> 8,      // Maximum value with last byte clear
        0x2020202020202020, // All spaces
        0x0101010101010101, // All SOH characters
        0x7F7F7F7F7F7F7F7F, // All DEL characters
    ];

    for &input in &test_cases {
        if let Some(decoded) = int_to_str(input) {
            let re_encoded = str_to_int(decoded).unwrap();
            assert_eq!(input, re_encoded, "Round-trip failed for: {:x}", input);
        }
    }
}

/// Test consecutive encoding/decoding
#[test]
fn test_consecutive_encoding() {
    let test_cases = ["test", "rust", "code", "1234"];

    for &input in &test_cases {
        // Encode and decode multiple times
        let mut current = input.to_string();
        for _ in 0..5 {
            let encoded = str_to_int(&current).unwrap();
            current = int_to_str(encoded).unwrap().to_string();
            assert_eq!(
                input, current,
                "Consecutive encoding/decoding failed for: {}",
                input
            );
        }
    }
}

/// Verify that find_null_byte_position works correctly
#[test]
fn test_null_byte_position() {
    // These tests are internal and require exposing the function or moving to the same module
    let test_cases = [
        (0u64, 0),                  // Empty string
        (0x41u64, 1),               // "A"
        (0x4142u64, 2),             // "AB"
        (0x414243u64, 3),           // "ABC"
        (0x41424344u64, 4),         // "ABCD"
        (0x4142434445u64, 5),       // "ABCDE"
        (0x414243444546u64, 6),     // "ABCDEF"
        (0x41424344454647u64, 7),   // "ABCDEFG"
        (0x4142434445464748u64, 8), // "ABCDEFGH"
    ];

    for (input, expected_len) in test_cases {
        if let Some(decoded) = int_to_str(input) {
            assert_eq!(
                decoded.len(),
                expected_len,
                "Incorrect length for input: {:x}",
                input
            );
        }
    }
}
