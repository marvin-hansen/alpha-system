use common::types::config::protocol::Protocol;

#[test]
fn test_default() {
    let protocol = Protocol::default();
    assert_eq!(protocol, Protocol::GRPC);
}

#[test]
fn test_debug() {
    let e1 = Protocol::GRPC;
    assert_eq!(format!("{:?}", e1), "GRPC");

    let e2 = Protocol::HTTP;
    assert_eq!(format!("{:?}", e2), "HTTP");

    let e3 = Protocol::UDP;
    assert_eq!(format!("{:?}", e3), "UDP");
}

#[test]
fn test_display() {
    let e1 = Protocol::GRPC;
    assert_eq!(format!("{}", e1), "GRPC");

    let e2 = Protocol::HTTP;
    assert_eq!(format!("{}", e2), "HTTP");

    let e3 = Protocol::UDP;
    assert_eq!(format!("{}", e3), "UDP");
}
