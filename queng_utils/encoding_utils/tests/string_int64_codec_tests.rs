use encoding_utils::{decode_int64_to_str, encode_str_to_int64};

#[test]
fn test_empty_string_encoding() {
    let encoded = encode_str_to_int64("").unwrap();
    assert_eq!(encoded, 0);

    let decoded = decode_int64_to_str(encoded).unwrap();
    assert_eq!("", decoded);
}

#[test]
fn test_single_char_encoding() {
    let test_cases = vec!["A", "B", "Z", "0", "9", "_", "a", "z"];

    for s in test_cases {
        let encoded = encode_str_to_int64(s).unwrap();
        let decoded = decode_int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_multi_char_encoding() {
    let test_cases = vec!["ABC", "123", "A1B2", "TEST_123", "ABCDEFGHIJ", "APPL"];

    for s in test_cases {
        let encoded = encode_str_to_int64(s).unwrap();
        let decoded = decode_int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_invalid_input() {
    // Too long string
    assert!(encode_str_to_int64("ABCDEFGHIJKLM").is_err());

    // Invalid characters
    assert!(encode_str_to_int64("A#B").is_err()); // special char
    assert!(encode_str_to_int64("!@#").is_err());
    assert!(encode_str_to_int64("%^&").is_err());
    assert!(encode_str_to_int64("*()_+").is_err());
    assert!(encode_str_to_int64("=-").is_err());
}

#[test]
fn test_deterministic() {
    let test_str = "APPL";
    let first_encoding = encode_str_to_int64(test_str).unwrap();

    // Encode multiple times to ensure same result
    for _ in 0..100 {
        let encoded = encode_str_to_int64(test_str).unwrap();
        assert_eq!(first_encoding, encoded);
    }

    let test_str = "BTCUSDT";
    let first_encoding = encode_str_to_int64(test_str).unwrap();

    // Encode multiple times to ensure same result
    for _ in 0..100 {
        let encoded = encode_str_to_int64(test_str).unwrap();
        assert_eq!(first_encoding, encoded);
    }
}

#[test]
fn test_max_length() {
    // Max length string
    let max_str = "ABCDEFGHIJ";
    let encoded = encode_str_to_int64(max_str).unwrap();
    let decoded = decode_int64_to_str(encoded).unwrap();
    assert_eq!(max_str, decoded);
}

#[test]
fn test_all_chars() {
    // All possible characters (keeping within 10 char limit)
    let all_chars = "AZaz09_123";
    let encoded = encode_str_to_int64(all_chars).unwrap();
    let decoded = decode_int64_to_str(encoded).unwrap();
    assert_eq!(all_chars, decoded);
}

#[test]
fn test_boundary_values() {
    // Test strings of different lengths (1 to MAX_LENGTH)
    let test_strs = vec![
        "A",          // 1 char
        "AB",         // 2 chars
        "ABC",        // 3 chars
        "ABCD",       // 4 chars (tests chunk processing)
        "ABCDE",      // 5 chars
        "ABCDEF",     // 6 chars
        "ABCDEFG",    // 7 chars
        "ABCDEFGH",   // 8 chars (tests multiple chunks)
        "ABCDEFGHI",  // 9 chars
        "ABCDEFGHIJ", // 10 chars (max length)
    ];

    for s in test_strs {
        let encoded = encode_str_to_int64(s).unwrap();
        let decoded = decode_int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_int_to_str_edge_cases() {
    // Test maximum valid value
    let max_str = "ZZZZZZZZZZ"; // 10 'Z's
    let encoded = encode_str_to_int64(max_str).unwrap();
    let decoded = decode_int64_to_str(encoded).unwrap();
    assert_eq!(max_str, decoded);

    // Test common trading pairs
    let pairs = vec!["BTCUSDT", "ETHUSDT", "BNBUSDT", "SOLUSDT"];
    for pair in pairs {
        let encoded = encode_str_to_int64(pair).unwrap();
        let decoded = decode_int64_to_str(encoded).unwrap();
        assert_eq!(pair, decoded);
    }
}

#[test]
fn test_chunk_processing() {
    // Test strings that exercise chunk processing boundaries
    let test_cases = vec![
        "ABCD",      // Exactly one chunk
        "ABCDE",     // One chunk plus remainder
        "ABCDEFGH",  // Exactly two chunks
        "ABCDEFGHI", // Two chunks plus remainder
    ];

    for s in test_cases {
        let encoded = encode_str_to_int64(s).unwrap();
        let decoded = decode_int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_mixed_characters() {
    let test_cases = vec!["aB1_", "Test123", "abcABC123"];
    for test_str in test_cases {
        let encoded = encode_str_to_int64(test_str).unwrap();
        let decoded = decode_int64_to_str(encoded).unwrap();
        assert_eq!(test_str, decoded);
    }
}

#[test]
fn test_encode_decode_roundtrip_special_cases() {
    let test_cases = vec![
        "A1B2C3D4",   // Mixed alphanumeric
        "98765432",   // All digits
        "ABCDEFGH",   // All uppercase
        "A",          // Single character
        "AB",         // Two characters
        "1234567890", // 10 characters
        "AAAAAAAAAA", // Repeated characters
    ];

    for input in test_cases {
        let encoded = encode_str_to_int64(input).expect("Failed to encode");
        let decoded = decode_int64_to_str(encoded).expect("Failed to decode");
        assert_eq!(input, decoded, "Round trip failed for input: {}", input);
    }
}

#[test]
fn test_encode_with_special_patterns() {
    let test_cases = vec![
        "10101010", // Alternating digits
        "ABABABAB", // Alternating letters
        "A1A1A1A1", // Alternating letter-digit
        "99999999", // Repeated digits
        "ZZZZZZZZ", // Repeated letters at boundary
    ];

    for input in test_cases {
        let result = encode_str_to_int64(input);
        assert!(result.is_ok(), "Failed to encode valid input: {}", input);

        // Verify round trip
        let encoded = result.unwrap();
        let decoded = decode_int64_to_str(encoded).expect("Failed to decode");
        assert_eq!(input, decoded, "Round trip failed for input: {}", input);
    }
}

#[test]
fn test_boundary_value_encoding() {
    // Test encoding of strings that should produce boundary values
    let test_cases = vec![
        "0",        // Minimum digit
        "9",        // Maximum digit
        "A",        // First letter
        "Z",        // Last letter
        "00000000", // All zeros
        "99999999", // All nines
        "AAAAAAAA", // All first letters
        "ZZZZZZZZ", // All last letters
    ];

    for input in test_cases {
        let encoded = encode_str_to_int64(input).expect("Failed to encode");
        let decoded = decode_int64_to_str(encoded).expect("Failed to decode");
        assert_eq!(input, decoded, "Round trip failed for input: {}", input);
    }
}
