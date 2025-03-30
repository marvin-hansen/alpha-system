// tests/heartbeat_tests.rs

use binary_protocol::{
    BytesSerializable, Commandable, HEARTBEAT_CODE, Heartbeat, Sizeable, Validatable,
};
use std::fmt::Write;

#[test]
fn test_heartbeat_new() {
    let heartbeat = Heartbeat::new();
    assert_eq!(heartbeat, Heartbeat {});
}

#[test]
fn test_heartbeat_default() {
    let heartbeat: Heartbeat = Default::default();
    assert_eq!(heartbeat, Heartbeat::new());
}

#[test]
fn test_heartbeat_command_code() {
    let heartbeat = Heartbeat::new();
    assert_eq!(heartbeat.command_code(), HEARTBEAT_CODE);
}

#[test]
fn test_heartbeat_byte_size() {
    let heartbeat = Heartbeat::new();
    assert_eq!(heartbeat.byte_size(), 0);
}

#[test]
fn test_heartbeat_validate() {
    let heartbeat = Heartbeat::new();
    assert!(heartbeat.validate().is_ok());
}

#[test]
fn test_heartbeat_display() {
    let heartbeat = Heartbeat::new();
    let mut output = String::new();
    write!(&mut output, "{}", heartbeat).unwrap();
    assert_eq!(output, "Heartbeat");
}

#[test]
fn test_heartbeat_partial_eq() {
    let heartbeat1 = Heartbeat::new();
    let heartbeat2 = Heartbeat::new();
    assert_eq!(heartbeat1, heartbeat2);
}

#[test]
fn test_heartbeat_serialize() {
    let heartbeat = Heartbeat::new();
    let bytes = heartbeat.to_bytes().expect("Serialization failed");
    let deserialized: Heartbeat = Heartbeat::from_bytes(&bytes).expect("Deserialization failed");
    assert_eq!(heartbeat, deserialized);
}
