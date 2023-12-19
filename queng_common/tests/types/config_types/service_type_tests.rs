use common::prelude::ServiceType;

#[test]
fn test_default() {
    let service_name = ServiceType::default();
    assert_eq!(service_name, ServiceType::ENDPOINT);
}

#[test]
fn test_from_i32() {
    assert_eq!(ServiceType::from(0x0_i32), ServiceType::NullVal);
    assert_eq!(ServiceType::from(0x1_i32), ServiceType::ENDPOINT);
    assert_eq!(ServiceType::from(0x2_i32), ServiceType::CHANNEL);
    assert_eq!(ServiceType::from(0x3_i32), ServiceType::NullVal);
}

#[test]
fn test_debug() {
    let e1 = ServiceType::ENDPOINT;
    assert_eq!(format!("{:?}", e1), "ENDPOINT");

    let e2 = ServiceType::CHANNEL;
    assert_eq!(format!("{:?}", e2), "CHANNEL");
}

#[test]
fn test_display() {
    let e1 = ServiceType::ENDPOINT;
    assert_eq!(format!("{}", e1), "ENDPOINT");

    let e2 = ServiceType::CHANNEL;
    assert_eq!(format!("{}", e2), "CHANNEL");

    let e3 = ServiceType::NullVal;
    assert_eq!(format!("{}", e3), "NullVal");
}
