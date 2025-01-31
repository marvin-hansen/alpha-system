/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::ClientOrderID;
use common_order_ext::ClientOrderIdExtension;

#[test]
fn test_encode_valid_client_order_id() {
    let client_order_id = ClientOrderID::from(String::from("ORDER123"));
    let result = client_order_id.encode_to_binary();
    assert!(result.is_ok());
}

#[test]
fn test_decode_valid_binary() {
    let encoded = 478560413000; // Known valid encoded value
    let result = ClientOrderID::decode_from_binary(encoded);
    assert!(result.is_ok());
}

#[test]
fn test_round_trip_encoding() {
    let original = ClientOrderID::from(String::from("ORDER123"));
    let encoded = original.clone().encode_to_binary().unwrap();
    let decoded = ClientOrderID::decode_from_binary(encoded).unwrap();
    assert_eq!(original.client_order_id(), decoded.client_order_id());
}

#[test]
#[should_panic(expected = "Client Order ID is empty")]
fn test_empty_string() {
    let _ = ClientOrderID::from(String::from(""));
}

#[test]
fn test_max_length_string() {
    let max_str = "A".repeat(10); // Maximum length for ClientOrderID
    let client_order_id = ClientOrderID::from(max_str);
    let result = client_order_id.encode_to_binary();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Client Order ID is too long")]
fn test_too_long_string() {
    let long_str = "A".repeat(11); // Exceeds maximum length
    let _ = ClientOrderID::from(long_str);
}

#[test]
fn test_alphanumeric() {
    let id = ClientOrderID::from(String::from("Test123"));
    let result = id.encode_to_binary();
    assert!(result.is_ok());
    let encoded = result.unwrap();
    let decoded = ClientOrderID::decode_from_binary(encoded).unwrap();
    assert_eq!("Test123", decoded.client_order_id());
}

#[test]
fn test_invalid_binary_decode() {
    // Using a value that would decode to an invalid string
    let result = ClientOrderID::decode_from_binary(1u64 << 63); // Set highest bit to force invalid char
    assert!(result.is_err());
}

#[test]
fn test_boundary_values() {
    // Test with valid boundary patterns
    let boundary_id = ClientOrderID::from(String::from("1234567890")); // 10 digits
    let result = boundary_id.encode_to_binary();
    assert!(result.is_ok());

    // Test with single character
    let min_id = ClientOrderID::from(String::from("1"));
    let result = min_id.encode_to_binary();
    assert!(result.is_ok());
}
