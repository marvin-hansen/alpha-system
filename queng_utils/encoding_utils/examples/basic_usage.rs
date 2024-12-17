use encoding_utils::{decode_int64_to_str, encode_str_to_int64};

fn main() {
    // Example 1: Basic encoding and decoding
    let input = "HELLO123";
    let encoded = encode_str_to_int64(input).expect("Failed to encode string");
    println!("Encoded '{}' to {}", input, encoded);

    let decoded = decode_int64_to_str(encoded).expect("Failed to decode integer");
    println!("Decoded {} back to '{}'", encoded, decoded);
    assert_eq!(input, decoded);

    // Example 2: Empty string
    let empty = "";
    let encoded = encode_str_to_int64(empty).expect("Failed to encode empty string");
    println!("\nEncoded empty string to {}", encoded);

    let decoded = decode_int64_to_str(encoded).expect("Failed to decode empty string");
    println!("Decoded {} back to empty string", encoded);
    assert_eq!(empty, decoded);

    // Example 3: Error handling - string too long
    let too_long = "ABCDEFGHIJKLMNOP";
    match encode_str_to_int64(too_long) {
        Ok(_) => println!("Unexpectedly encoded too long string!"),
        Err(e) => println!(
            "Correctly rejected too long string: '{}' with error: {}",
            too_long, e
        ),
    }

    // Example 4: Error handling - non-ASCII characters
    let non_ascii = "Hello世界";
    match encode_str_to_int64(non_ascii) {
        Ok(_) => println!("Unexpectedly encoded non-ASCII string!"),
        Err(e) => println!(
            "Correctly rejected non-ASCII string: '{}' with error: {}",
            non_ascii, e
        ),
    }

    // Example 5: Error handling - invalid characters
    let invalid_chars = "Hello-World";
    match encode_str_to_int64(invalid_chars) {
        Ok(_) => println!("Unexpectedly encoded string with invalid characters!"),
        Err(e) => println!(
            "Correctly rejected string with invalid characters: '{}' with error: {}",
            invalid_chars, e
        ),
    }

    // Example 6: Error handling - invalid encoded value
    let invalid_encoded = u64::MAX;
    match decode_int64_to_str(invalid_encoded) {
        Ok(_) => println!("Unexpectedly decoded invalid integer!"),
        Err(e) => println!(
            "Correctly rejected invalid encoded value: {} with error: {}",
            invalid_encoded, e
        ),
    }
}
