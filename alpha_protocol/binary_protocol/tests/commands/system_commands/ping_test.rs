use binary_protocol::{BytesSerializable, Ping, Sizeable, Validatable};

const BASE_SIZE: usize = 0;

#[test]
fn test_ping_validation() {
    let ping = Ping::new();

    let result = ping.validate();

    assert!(result.is_ok());
}

#[test]
fn test_ping_byte_size() {
    let ping = Ping::new();

    let size = ping.byte_size();

    assert_eq!(size, BASE_SIZE);
}

#[test]
fn test_ping_display() {
    let ping = Ping::new();

    let display = format!("{}", ping);

    assert_eq!(display, "Ping");
}

#[test]
fn test_to_bytes_from_bytes() {
    let ping = Ping::new();

    let bytes = ping.to_bytes().unwrap();
    // Yeap, zero bytes for zero sized struct.
    assert_eq!(bytes.len(), 0);

    let deserialized: Ping = Ping::from_bytes(&bytes).unwrap();

    // Test equality
    assert_eq!(ping, deserialized);
}
