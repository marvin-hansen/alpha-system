use encoding_utils::{int64_to_str, str_to_int64};

fn main() {
    // Basic encoding and decoding
    let input = "hello";
    println!("Original string: {}", input);

    // Convert string to integer
    let encoded = str_to_int64(input).expect("Failed to encode string");
    println!("Encoded as integer: {}", encoded);

    // Convert back to string
    let decoded = int64_to_str(encoded).expect("Failed to decode integer");
    println!("Decoded string: {}", decoded);
    assert_eq!(input, decoded);

    // Handling empty strings
    let empty = "";
    let encoded_empty = str_to_int64(empty).expect("Failed to encode empty string");
    let decoded_empty = int64_to_str(encoded_empty).expect("Failed to decode empty string");
    assert_eq!(empty, decoded_empty);
    println!("\nEmpty string handling:");
    println!("Empty string -> {} -> {:?}", encoded_empty, decoded_empty);

    // Maximum length strings
    let max_length = "12345678";
    let encoded_max = str_to_int64(max_length).expect("Failed to encode max length string");
    let decoded_max = int64_to_str(encoded_max).expect("Failed to decode max length string");
    assert_eq!(max_length, decoded_max);
    println!("\nMaximum length string handling:");
    println!("'{}' -> {} -> {}", max_length, encoded_max, decoded_max);

    // Error handling examples
    println!("\nError handling examples:");

    // Too long string
    let too_long = "123456789";
    match str_to_int64(too_long) {
        Some(_) => println!("Unexpectedly encoded too long string!"),
        None => println!("Correctly rejected too long string: '{}'", too_long),
    }

    // Non-ASCII string
    let non_ascii = "hello❤";
    match str_to_int64(non_ascii) {
        Some(_) => println!("Unexpectedly encoded non-ASCII string!"),
        None => println!("Correctly rejected non-ASCII string: '{}'", non_ascii),
    }

    // Invalid encoded integer
    let invalid_encoded = 0xFF00_0000_0000_0000u64; // Contains non-ASCII bytes
    match int64_to_str(invalid_encoded) {
        Some(_) => println!("Unexpectedly decoded invalid integer!"),
        None => println!(
            "Correctly rejected invalid encoded integer: {:#x}",
            invalid_encoded
        ),
    }
}
