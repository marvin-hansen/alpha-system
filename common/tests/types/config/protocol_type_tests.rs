use common::prelude::ProtocolType;

#[test]
fn test_default() {
    let protocol = ProtocolType::default();
    assert_eq!(protocol, ProtocolType::GRPC);
}

#[test]
fn test_debug() {
    let e1 = ProtocolType::GRPC;
    assert_eq!(format!("{:?}", e1), "GRPC");

    let e2 = ProtocolType::HTTP;
    assert_eq!(format!("{:?}", e2), "HTTP");

    let e3 = ProtocolType::UDP;
    assert_eq!(format!("{:?}", e3), "UDP");
}

#[test]
fn test_display() {
    let e1 = ProtocolType::GRPC;
    assert_eq!(format!("{}", e1), "GRPC");

    let e2 = ProtocolType::HTTP;
    assert_eq!(format!("{}", e2), "HTTP");

    let e3 = ProtocolType::UDP;
    assert_eq!(format!("{}", e3), "UDP");
}
