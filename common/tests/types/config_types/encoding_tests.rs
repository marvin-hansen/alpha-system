use common::prelude::Encoding;

#[test]
fn test_default() {
    let protocol = Encoding::default();
    assert_eq!(protocol, Encoding::Protobuf);
}

#[test]
fn test_debug() {
    let e1 = Encoding::Protobuf;
    assert_eq!(format!("{:?}", e1), "Protobuf");

    let e2 = Encoding::SBE;
    assert_eq!(format!("{:?}", e2), "SBE");
}

#[test]
fn test_display() {
    let e1 = Encoding::Protobuf;
    assert_eq!(format!("{}", e1), "Protobuf");

    let e2 = Encoding::SBE;
    assert_eq!(format!("{}", e2), "SBE");
}
