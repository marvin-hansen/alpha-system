use encoding_utils::{decode_int128_to_str, encode_str_to_int128};

#[test]
fn test_empty_string() {
    let result = encode_str_to_int128("");
    assert!(result.is_ok());
    let encoded = result.unwrap();
    assert_eq!(encoded, 0, "Empty string should encode to 0");

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
        assert!(
            encoded > 0,
            "Encoded value should be positive for non-empty string"
        );

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
    assert!(result.unwrap_err().to_string().contains("too long"));
}

#[test]
fn test_invalid_characters() {
    let invalid_cases = vec![
        "Hello World", // space is invalid
        "Hello!",      // ! is invalid
        "Test@Case",   // @ is invalid
        "123-456",     // - is invalid
        "Tab\tHere",   // tab is invalid
        "New\nLine",   // newline is invalid
        "Special#$%",  // multiple special chars
        "Unicode→→",   // unicode arrows
    ];

    for test_str in invalid_cases {
        let result = encode_str_to_int128(test_str);
        assert!(
            result.is_err(),
            "Expected error for invalid string: {}",
            test_str
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid character"),
            "Error message should mention invalid character"
        );
    }
}

#[test]
fn test_bit_patterns() {
    // Test different bit patterns to ensure no bit loss
    let test_cases = vec![
        "AAAAAAAAAA",  // All same character
        "ABCDEFGHIJ",  // Sequential uppercase
        "9876543210",  // Reverse sequential numbers
        "A1B2C3D4E5",  // Alternating patterns
        "_________9",  // Edge cases in encoding
        "abcdefghij",  // Sequential lowercase
        "aAbBcCdDeE",  // Alternating case
        "0000000000",  // All zeros
        "9999999999",  // All nines
        "ZZZZZZZZZZ",  // All highest valid char
        "a1_A2_b3_B4", // Mixed with underscores
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
    let test_cases = vec![
        "aA1bB2cC3",
        "TestTest123",
        "UPPER123lower",
        "123ABC456def",
        "MiXeDcAsE789",
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
fn test_boundary_cases() {
    // Test single character for each valid type
    for c in b'A'..=b'Z' {
        let test_str = (c as char).to_string();
        let encoded = encode_str_to_int128(&test_str).unwrap();
        let decoded = decode_int128_to_str(encoded).unwrap();
        assert_eq!(test_str, decoded, "Failed for uppercase {}", test_str);
    }

    for c in b'a'..=b'z' {
        let test_str = (c as char).to_string();
        let encoded = encode_str_to_int128(&test_str).unwrap();
        let decoded = decode_int128_to_str(encoded).unwrap();
        assert_eq!(test_str, decoded, "Failed for lowercase {}", test_str);
    }

    for c in b'0'..=b'9' {
        let test_str = (c as char).to_string();
        let encoded = encode_str_to_int128(&test_str).unwrap();
        let decoded = decode_int128_to_str(encoded).unwrap();
        assert_eq!(test_str, decoded, "Failed for digit {}", test_str);
    }
}

#[test]
fn test_edge_length_cases() {
    // Test strings of various lengths up to max
    for len in 1..=20 {
        let test_str = "A".repeat(len);
        let result = encode_str_to_int128(&test_str);
        assert!(result.is_ok(), "Failed to encode length {}", len);
        let encoded = result.unwrap();

        let result = decode_int128_to_str(encoded);
        assert!(result.is_ok(), "Failed to decode length {}", len);
        let decoded = result.unwrap();
        assert_eq!(test_str, decoded, "Failed for length {}", len);
    }
}

#[test]
fn test_encode_decode_roundtrip_special_cases() {
    let test_cases = vec![
        "A1B2C3D4E5",      // Mixed alphanumeric
        "9876543210",      // All digits
        "ABCDEFGHIJ",      // All uppercase
        "A",               // Single character
        "AB",              // Two characters
        "123456789012345", // 15 characters
        "AAAAAAAAAAAAAAA", // Repeated characters
    ];

    for input in test_cases {
        let encoded = encode_str_to_int128(input).expect("Failed to encode");
        let decoded = decode_int128_to_str(encoded).expect("Failed to decode");
        assert_eq!(input, decoded, "Round trip failed for input: {}", input);
    }
}

#[test]
fn test_encode_with_special_patterns() {
    let test_cases = vec![
        "101010101",  // Alternating digits
        "ABABABABAB", // Alternating letters
        "A1A1A1A1A1", // Alternating letter-digit
        "999999999",  // Repeated digits
        "ZZZZZZZZZ",  // Repeated letters at boundary
    ];

    for input in test_cases {
        let result = encode_str_to_int128(input);
        assert!(result.is_ok(), "Failed to encode valid input: {}", input);

        // Verify round trip
        let encoded = result.unwrap();
        let decoded = decode_int128_to_str(encoded).expect("Failed to decode");
        assert_eq!(input, decoded, "Round trip failed for input: {}", input);
    }
}

#[test]
fn test_edge_case_combinations() {
    // Test combinations of valid characters at different positions
    let test_cases = vec![
        "A9B8C7D6E5", // Alternating letters and numbers
        "12345ABCDE", // Numbers followed by letters
        "ABCDE12345", // Letters followed by numbers
        "A1B2C3D4E5", // Interleaved letters and numbers
    ];

    for input in test_cases {
        let encoded = encode_str_to_int128(input).expect("Failed to encode");
        let decoded = decode_int128_to_str(encoded).expect("Failed to decode");
        assert_eq!(input, decoded, "Round trip failed for input: {}", input);
    }
}
