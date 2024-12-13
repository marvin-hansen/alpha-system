#[cfg(test)]
mod tests {
    use encoding_utils::*;

    #[test]
    fn test_invalid_inputs() {
        // Test lowercase letters
        assert_eq!(str_to_int("hello"), None);
        assert_eq!(str_to_int("World"), None);

        // Test special characters
        assert_eq!(str_to_int("!@#$"), None);
        assert_eq!(str_to_int("A!B"), None);
        assert_eq!(str_to_int("AB C"), None); // space
        assert_eq!(str_to_int("AB\nC"), None); // newline
        assert_eq!(str_to_int("AB\tC"), None); // tab

        // Test Unicode characters
        assert_eq!(str_to_int("HELLO🌍"), None);
        assert_eq!(str_to_int("CAFÉ"), None);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty string and zero
        assert_eq!(str_to_int(""), Some(0));
        assert_eq!(int_to_str(0).unwrap(), "");

        // Test single characters
        assert_eq!(str_to_int("A").unwrap(), 0);
        assert_eq!(str_to_int("B").unwrap(), 1);
        assert_eq!(str_to_int("C").unwrap(), 2);
        assert_eq!(int_to_str(0).unwrap(), "");
        assert_eq!(int_to_str(1).unwrap(), "B");
        assert_eq!(int_to_str(2).unwrap(), "C");

        // Test two characters
        assert_eq!(str_to_int("BA").unwrap(), 37);
        assert_eq!(int_to_str(37).unwrap(), "BA");
        assert_eq!(str_to_int("BB").unwrap(), 38);
        assert_eq!(int_to_str(38).unwrap(), "BB");

        // Test maximum length string
        let max_str = "B".repeat(MAX_CHARS as usize);
        let encoded = str_to_int(&max_str);
        assert!(encoded.is_some(), "Should handle maximum valid length");
        let decoded = int_to_str(encoded.unwrap()).unwrap();
        assert_eq!(decoded.len(), max_str.len());
    }

    #[test]
    fn test_determinism() {
        let test_cases = vec![
            "B", // Start with B since A encodes to 0 which decodes to ""
            "Z", "0", "9", "_", "HELLO", "WORLD", "TEST", "BCD123", "XYZ789", "B_C",
        ];

        for input in test_cases {
            let encoded = str_to_int(input).unwrap();
            // Test multiple encodings of the same string
            for _ in 0..10 {
                assert_eq!(
                    str_to_int(input).unwrap(),
                    encoded,
                    "Encoding not deterministic for {}",
                    input
                );
                assert_eq!(
                    int_to_str(encoded).unwrap(),
                    input,
                    "Decoding not deterministic for {}",
                    input
                );
            }
        }
    }

    #[test]
    fn test_valid_encoding_decoding() {
        let cases = vec![
            ("", 0),
            ("A", 0),
            ("B", 1),
            ("C", 2),
            ("Z", 25),
            ("0", 26),
            ("9", 35),
            ("_", 36),
            ("BA", 37),
            ("BB", 38),
            ("BC", 39),
            ("CA", 74),
            ("HELLO", 41964367),
            ("WORLD", 41964367),
        ];

        for (s, expected) in cases {
            let encoded = str_to_int(s).unwrap();
            assert_eq!(encoded, expected, "Encoding failed for {}", s);
            let decoded = int_to_str(encoded).unwrap();
            if encoded == 0 {
                assert_eq!(decoded, "", "Decoding failed for zero value");
            } else {
                assert_eq!(decoded, s, "Decoding failed for {}", s);
            }
        }
    }

    #[test]
    fn test_all_valid_chars() {
        // Test all uppercase letters
        for c in b'A'..=b'Z' {
            let s = String::from_utf8(vec![c]).unwrap();
            let encoded = str_to_int(&s).unwrap();
            let decoded = int_to_str(encoded).unwrap();
            if encoded == 0 {
                assert_eq!(decoded, "", "Failed for zero value");
            } else {
                assert_eq!(decoded, s, "Failed for uppercase letter {}", s);
            }
        }

        // Test all digits
        for c in b'0'..=b'9' {
            let s = String::from_utf8(vec![c]).unwrap();
            let encoded = str_to_int(&s).unwrap();
            let decoded = int_to_str(encoded).unwrap();
            assert_eq!(s, decoded, "Failed for digit {}", s);
        }

        // Test underscore
        let encoded = str_to_int("_").unwrap();
        let decoded = int_to_str(encoded).unwrap();
        assert_eq!("_", decoded, "Failed for underscore");
    }

    #[test]
    fn test_consecutive_operations() {
        let test_cases = vec![
            "B", "Z", "0", "9", "_", // Start with B since A encodes to 0
            "HELLO", "WORLD", "TEST", "BCD123", "XYZ789", "B_C",
        ];

        for input in test_cases {
            let mut current = input.to_string();
            // Perform multiple encode/decode cycles
            for i in 0..5 {
                let encoded = str_to_int(&current).unwrap();
                current = int_to_str(encoded).unwrap();
                assert_eq!(
                    current,
                    input,
                    "Failed after {} encode/decode cycles for {}",
                    i + 1,
                    input
                );
            }
        }
    }
}
