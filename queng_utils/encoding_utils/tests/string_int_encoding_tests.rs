use encoding_utils::{int64_to_str, str_to_int64};

#[test]
fn test_empty_string_encoding() {
    let encoded = str_to_int64("").unwrap();
    assert_eq!(encoded, 0);

    let decoded = int64_to_str(encoded).unwrap();
    assert_eq!("", decoded);
}

#[test]
fn test_single_char_encoding() {
    let test_cases = vec!["A", "B", "Z", "0", "9", "_", "-"];

    for s in test_cases {
        let encoded = str_to_int64(s).unwrap();
        let decoded = int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_multi_char_encoding() {
    let test_cases = vec!["ABC", "123", "A1B2", "TEST_123", "ABCDEFGHIJ", "APPL"];

    for s in test_cases {
        let encoded = str_to_int64(s).unwrap();
        let decoded = int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_invalid_input() {
    // Too long string
    assert!(str_to_int64("ABCDEFGHIJKLM").is_none());

    // Invalid characters
    assert!(str_to_int64("abc").is_none()); // lowercase
    assert!(str_to_int64("A B").is_none()); // space
    assert!(str_to_int64("A#B").is_none()); // special char

    // Special characters
    assert!(str_to_int64("!@#").is_none());
    assert!(str_to_int64("%^&").is_none());
    assert!(str_to_int64("*()_+").is_none());
    assert!(str_to_int64("=-").is_none());
}

#[test]
fn test_deterministic() {
    let test_str = "APPL";
    let first_encoding = str_to_int64(test_str).unwrap();

    // Encode multiple times to ensure same result
    for _ in 0..100 {
        let encoded = str_to_int64(test_str).unwrap();
        assert_eq!(first_encoding, encoded);
    }

    let test_str = "BTCUSDT";
    let first_encoding = str_to_int64(test_str).unwrap();

    // Encode multiple times to ensure same result
    for _ in 0..100 {
        let encoded = str_to_int64(test_str).unwrap();
        assert_eq!(first_encoding, encoded);
    }
}

#[test]
fn test_max_length() {
    // Max length string
    let max_str = "ABCDEFGHIJ";
    let encoded = str_to_int64(max_str).unwrap();
    let decoded = int64_to_str(encoded).unwrap();
    assert_eq!(max_str, decoded);
}

#[test]
fn test_all_chars() {
    // All possible characters
    let all_chars = "ABCZ0123_-";
    let encoded = str_to_int64(all_chars).unwrap();
    let decoded = int64_to_str(encoded).unwrap();
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
        let encoded = str_to_int64(s).unwrap();
        let decoded = int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_int_to_str_edge_cases() {
    // Test maximum valid value
    let max_str = "ZZZZZZZZZZ"; // 10 'Z's
    let encoded = str_to_int64(max_str).unwrap();
    let decoded = int64_to_str(encoded).unwrap();
    assert_eq!(max_str, decoded);

    // Test common trading pairs
    let pairs = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT"];
    for pair in pairs {
        let encoded = str_to_int64(pair).unwrap();
        let decoded = int64_to_str(encoded).unwrap();
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
        let encoded = str_to_int64(s).unwrap();
        let decoded = int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}

#[test]
fn test_mixed_characters() {
    // Test mixing different character types
    let test_cases = vec!["A1B2C3", "123ABC", "ABC123", "A_B-C1", "1-2_3A"];

    for s in test_cases {
        let encoded = str_to_int64(s).unwrap();
        let decoded = int64_to_str(encoded).unwrap();
        assert_eq!(s, decoded);
    }
}
