use common_order::ExchangeOrderID;
use common_order_ext::ExchangeOrderIdExtension;

#[test]
fn test_encode_valid_exchange_order_id() {
    let exchange_order_id = ExchangeOrderID::from(String::from("ORDER123"));
    let result = exchange_order_id.encode_to_binary();
    assert!(result.is_ok());
}

#[test]
fn test_decode_valid_binary() {
    let encoded = (478560413000, 0); // Known valid encoded value
    let result = ExchangeOrderID::decode_from_binary(encoded);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert!(!decoded.exchange_order_id().is_empty());
}

#[test]
fn test_round_trip_encoding() {
    let original = ExchangeOrderID::from(String::from("ORDER123"));
    let encoded = original.clone().encode_to_binary().unwrap();
    let decoded = ExchangeOrderID::decode_from_binary(encoded).unwrap();
    assert_eq!(original.exchange_order_id(), decoded.exchange_order_id());
}

#[test]
fn test_empty_string_decode() {
    // Using a value that would decode to an empty string
    let result = ExchangeOrderID::decode_from_binary((0, 0));
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Binary decoding error: Exchange Order ID is empty"
    );
}

#[test]
fn test_max_length_string() {
    let max_str = "A".repeat(20); // Maximum length for ExchangeOrderID
    let exchange_order_id = ExchangeOrderID::from(max_str);
    let result = exchange_order_id.encode_to_binary();
    assert!(result.is_ok());
}

#[test]
fn test_too_long_string_decode() {
    // Create a binary value that would decode to a string longer than 20 chars
    let long_str = "A".repeat(20); // Create a valid length string first
    let mut encoded = encode_str_to_pair_u64(&long_str).unwrap();
    // Modify the second u64 to make it decode to a longer string
    encoded.1 = u64::MAX;
    let result = ExchangeOrderID::decode_from_binary(encoded);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Binary decoding error: Invalid encoded value: contains bits outside of valid range"
    );
}

#[test]
fn test_alphanumeric() {
    let id = ExchangeOrderID::from(String::from("Test123_XYZ"));
    let result = id.clone().encode_to_binary();
    assert!(result.is_ok());
    let encoded = result.unwrap();
    let decoded = ExchangeOrderID::decode_from_binary(encoded).unwrap();
    assert_eq!(id.exchange_order_id(), decoded.exchange_order_id());
}

#[test]
fn test_invalid_binary_decode() {
    // Using a value that would decode to an invalid string
    let result = ExchangeOrderID::decode_from_binary((u64::MAX, u64::MAX)); // Invalid binary data
    assert!(result.is_err());
}

#[test]
fn test_boundary_values() {
    // Test with maximum length
    let max_id = ExchangeOrderID::from("12345678901234567890".to_string()); // 20 chars
    let result = max_id.clone().encode_to_binary();
    assert!(result.is_ok());

    // Test with single character
    let min_id = ExchangeOrderID::from("1".to_string());
    let result = min_id.encode_to_binary();
    assert!(result.is_ok());

    // Test round trip with max length
    let encoded = max_id.clone().encode_to_binary().unwrap();
    let decoded = ExchangeOrderID::decode_from_binary(encoded).unwrap();
    assert_eq!(max_id.exchange_order_id(), decoded.exchange_order_id());
}

// Need to import for the test_too_long_string_decode test
use encoding_utils::encode_str_to_pair_u64;
