use messages::prelude::MessageType;

#[test]
fn test_message_type_from() {
    assert_eq!(MessageType::from(1), MessageType::StartData);
    assert_eq!(MessageType::from(2), MessageType::StopData);
    assert_eq!(MessageType::from(0), MessageType::NullVal);
    // Unknown message type results in NullVal
    assert_eq!(MessageType::from(232), MessageType::NullVal);
}

#[test]
fn test_message_type_display() {
    assert_eq!(format!("{}", MessageType::StartData), "StartData");
    assert_eq!(format!("{}", MessageType::StopData), "StopData");
    assert_eq!(format!("{}", MessageType::NullVal), "NullVal");
}
