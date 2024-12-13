use encoding_utils::{lookup_char, lookup_u64};

#[test]
fn test_encoding_decoding() {
    let test_cases = [
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('0', 27),
        ('1', 28),
        ('2', 29),
        ('9', 36),
        ('_', 37),
        ('-', 38),
    ];

    for (char, expected) in test_cases {
        let encoded = lookup_u64(char as u8);
        assert_eq!(encoded, expected);

        let decoded = lookup_char(encoded);
        assert_eq!(decoded, char);
    }
}

#[test]
#[should_panic]
fn test_lookup_u64_out_of_bounds() {
    lookup_u64(255);
}

#[test]
#[should_panic]
fn test_lookup_char_out_of_bounds() {
    lookup_char(256);
}
