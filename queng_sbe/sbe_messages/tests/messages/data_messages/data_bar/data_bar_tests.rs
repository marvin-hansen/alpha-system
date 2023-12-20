use common::prelude::DataBar;
use sbe_messages::prelude::SbeDataBar;

#[test]
fn test_encode_data_bar_message() {
    let bar = DataBar::default(); // Create a sample DataBar

    let result = SbeDataBar::encode_data_bar_message(bar);

    assert!(result.is_ok()); // Assert encode passes

    let (size, encoded) = result.unwrap();
    assert_eq!(size, 39); // Assert encoded message size matches expected
    assert!(!encoded.is_empty()); // Assert non-empty encoded message
}

#[test]
fn test_decode_data_bar_message() {
    // Encode a sample DataBar
    let bar = DataBar::default();
    let (size, encoded) = SbeDataBar::encode_data_bar_message(bar.clone()).unwrap();
    assert_eq!(size, 39); // Assert encoded message size matches expected
    assert!(!encoded.is_empty()); // Assert non-empty encoded message

    // Decode the encoded message
    let result = SbeDataBar::decode_data_bar_message(&encoded);

    assert!(result.is_ok()); // Assert decode passes

    let decoded = result.unwrap();
    assert_eq!(decoded, bar.clone()); // Assert decoded bar matches original
}
