use encoding_utils::{decode_pair_64_to_str, encode_str_to_pair_u64};

#[test]
fn test_empty_string() {
    // Test encoding empty string
    let result = encode_str_to_pair_u64("");
    assert!(result.is_ok());
    let encoded = result.unwrap();
    assert_eq!((0, 0), encoded);

    // Test decoding empty string
    let result = decode_pair_64_to_str((0, 0));
    assert!(result.is_ok());
    assert_eq!("", result.unwrap());
}

#[test]
fn test_basic_encoding_decoding() {
    let test_cases = vec![
        "Hello_World123",
        "ABCDEFGHIJ",
        "abcdefghij",
        "0123456789",
        "Test_123_ABC",
    ];

    for test_str in test_cases {
        let result = encode_str_to_pair_u64(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();

        let result = decode_pair_64_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {:?}", encoded);
        let decoded = result.unwrap();

        assert_eq!(test_str, decoded, "String mismatch after encode/decode");
    }
}

#[test]
fn test_max_length() {
    // Test maximum length string (20 characters)
    let max_str = "ABCDEFGHIJ0123456789";
    let result = encode_str_to_pair_u64(max_str);
    assert!(result.is_ok());
    let encoded = result.unwrap();

    let result = decode_pair_64_to_str(encoded);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(max_str, decoded);

    // Test string too long (21 characters)
    let too_long = "ABCDEFGHIJ0123456789X";
    let result = encode_str_to_pair_u64(too_long);
    assert!(result.is_err());
}

#[test]
fn test_invalid_characters() {
    let invalid_cases = vec![
        "Hello World", // space is invalid
        "Hello!",      // ! is invalid
        "Test@Case",   // @ is invalid
        "123-456",     // - is invalid
    ];

    for test_str in invalid_cases {
        let result = encode_str_to_pair_u64(test_str);
        assert!(
            result.is_err(),
            "Expected error for invalid string: {}",
            test_str
        );
    }
}

#[test]
fn test_boundary_values() {
    // Test first half only (0-9 characters)
    let test_cases = vec![
        "A",
        "AB",
        "ABC",
        "ABCD",
        "ABCDE",
        "ABCDEF",
        "ABCDEFG",
        "ABCDEFGH",
        "ABCDEFGHI",
    ];

    for test_str in test_cases {
        let result = encode_str_to_pair_u64(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();
        assert_eq!(encoded.1, 0, "Second u64 should be 0 for short strings");

        let result = decode_pair_64_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {:?}", encoded);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded);
    }

    // Test second half (11-20 characters)
    let test_cases = vec![
        "ABCDEFGHIJK",
        "ABCDEFGHIJKL",
        "ABCDEFGHIJKLM",
        "ABCDEFGHIJKLMN",
        "ABCDEFGHIJKLMNO",
        "ABCDEFGHIJKLMNOP",
        "ABCDEFGHIJKLMNOPQ",
        "ABCDEFGHIJKLMNOPQR",
        "ABCDEFGHIJKLMNOPQRS",
        "ABCDEFGHIJKLMNOPQRST",
    ];

    for test_str in test_cases {
        let result = encode_str_to_pair_u64(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();
        assert_ne!(encoded.1, 0, "Second u64 should not be 0 for long strings");

        let result = decode_pair_64_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {:?}", encoded);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded);
    }
}

#[test]
fn test_bit_patterns() {
    // Test strings with specific bit patterns
    let test_cases = vec![
        "_________9", // All underscores except last char
        "AAAAAAAAA0", // Repeated same character
        "0000000000", // All zeros
        "ZZZZZZZZZZ", // All highest value chars
    ];

    for test_str in test_cases {
        let result = encode_str_to_pair_u64(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();

        let result = decode_pair_64_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {:?}", encoded);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded, "String mismatch after encode/decode");
    }
}

#[test]
fn test_mixed_case() {
    // Test mixed case handling
    let test_cases = vec!["aAbBcCdDeE", "TestTest123", "MiXeDcAsE_"];

    for test_str in test_cases {
        let result = encode_str_to_pair_u64(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();

        let result = decode_pair_64_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {:?}", encoded);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded);
    }
}

#[test]
fn test_invalid_encoded_values() {
    // Test decoding invalid encoded values
    let invalid_cases = vec![
        (1u64 << 63, 0), // Value too large for 6 bits
        (0, 1u64 << 63), // Value too large in second u64
        (u64::MAX, 0),   // All bits set in first u64
        (0, u64::MAX),   // All bits set in second u64
    ];

    for encoded in invalid_cases {
        let result = decode_pair_64_to_str(encoded);
        assert!(
            result.is_err(),
            "Expected error for invalid encoded value: {:?}",
            encoded
        );
    }
}

#[test]
fn test_deterministic() {
    // Test that encoding is deterministic
    let test_cases = vec![
        "Hello_World123",
        "ABCDEFGHIJKLMNOPQRST",
        "0123456789",
        "aAbBcCdDeE",
    ];

    for test_str in test_cases {
        let first_encode = encode_str_to_pair_u64(test_str).unwrap();
        let second_encode = encode_str_to_pair_u64(test_str).unwrap();
        assert_eq!(
            first_encode, second_encode,
            "Encoding should be deterministic"
        );

        let first_decode = decode_pair_64_to_str(first_encode).unwrap();
        let second_decode = decode_pair_64_to_str(second_encode).unwrap();
        assert_eq!(
            first_decode, second_decode,
            "Decoding should be deterministic"
        );
        assert_eq!(
            test_str, first_decode,
            "Original string should match decoded"
        );
    }
}
