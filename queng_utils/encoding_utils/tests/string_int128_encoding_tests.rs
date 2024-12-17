use encoding_utils::{decode_int128_to_str, encode_str_to_int128};

#[test]
fn test_empty_string() {
    let result = encode_str_to_int128("");
    assert!(result.is_ok());
    let encoded = result.unwrap();

    let result = decode_int128_to_str(encoded);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!("", decoded);
}

#[test]
fn test_basic_encoding_decoding() {
    let test_cases = vec![
        "Hello_World123",
        "ABCDEFGHIJ",
        "abcdefghij",
        "0123456789",
        "Test_Case_123",
        "a",
        "Z",
        "9",
        "_",
    ];

    for test_str in test_cases {
        let result = encode_str_to_int128(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();

        let result = decode_int128_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {}", test_str);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded, "String mismatch after encode/decode");
    }
}

#[test]
fn test_max_length() {
    let max_str = "ABCDEFGHIJ0123456789"; // 20 characters
    let result = encode_str_to_int128(max_str);
    assert!(result.is_ok());
    let encoded = result.unwrap();

    let result = decode_int128_to_str(encoded);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(max_str, decoded);

    // Test string too long
    let too_long = "ABCDEFGHIJ0123456789X"; // 21 characters
    let result = encode_str_to_int128(too_long);
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
        let result = encode_str_to_int128(test_str);
        assert!(
            result.is_err(),
            "Expected error for invalid string: {}",
            test_str
        );
    }
}

#[test]
fn test_bit_patterns() {
    // Test different bit patterns to ensure no bit loss
    let test_cases = vec![
        "AAAAAAAAAA", // All same character
        "ABCDEFGHIJ", // Sequential characters
        "9876543210", // Reverse sequential
        "A1B2C3D4E5", // Alternating patterns
        "_________9", // Edge cases in encoding
    ];

    for test_str in test_cases {
        let result = encode_str_to_int128(test_str);
        assert!(result.is_ok(), "Failed to encode: {}", test_str);
        let encoded = result.unwrap();

        let result = decode_int128_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode: {}", test_str);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded, "String mismatch after encode/decode");
    }
}

#[test]
fn test_mixed_case() {
    let test_str = "aAbBcCdDeE";
    let result = encode_str_to_int128(test_str);
    assert!(result.is_ok());
    let encoded = result.unwrap();

    let result = decode_int128_to_str(encoded);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(test_str, decoded);
}
