use binary_protocol::{BytesSerializable, Commandable, PONG_CODE, Pong, Sizeable, Validatable};

#[test]
fn test_new_and_default() {
    let pong1 = Pong::new();
    let pong2 = Pong::default();

    assert_eq!(pong1, pong2);
}

#[test]
fn test_command_code() {
    let pong = Pong::new();
    assert_eq!(pong.command_code(), PONG_CODE);
}

#[test]
fn test_byte_size() {
    let pong = Pong::new();
    assert_eq!(pong.byte_size(), 0);
}

#[test]
fn test_validate() {
    let pong = Pong::new();
    assert!(pong.validate().is_ok());
}

#[test]
fn test_display() {
    let pong = Pong::new();
    assert_eq!(format!("{}", pong), "Pong");
}

#[test]
fn test_serialization_deserialization() {
    let pong = Pong::new();
    let serialized = pong.to_bytes().unwrap();

    // Serialized data should be empty (0 bytes)
    assert_eq!(serialized.len(), 0);

    // Deserialize and verify
    let deserialized = Pong::from_bytes(&serialized).unwrap();
    assert_eq!(pong, deserialized);
}

#[test]
fn test_clone() {
    let pong = Pong::new();
    let cloned = pong.clone();
    assert_eq!(pong, cloned);
}

#[test]
fn test_debug() {
    let pong = Pong::new();
    assert_eq!(format!("{:?}", pong), "Pong");
}
