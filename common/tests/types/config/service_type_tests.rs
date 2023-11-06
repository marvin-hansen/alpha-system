use common::prelude::ServiceType;

#[test]
fn test_default() {
    let service_name = ServiceType::default();
    assert_eq!(service_name, ServiceType::ENDPOINT);
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
}
