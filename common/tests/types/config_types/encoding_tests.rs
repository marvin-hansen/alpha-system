use common::prelude::Encoding;

#[test]
fn test_default() {
    let protocol = Encoding::default();
    assert_eq!(protocol, Encoding::NullVal);
}

#[test]
fn test_debug() {
    let e = Encoding::Binary;
    assert_eq!(format!("{:?}", e), "Binary");

    let e1 = Encoding::Protobuf;
    assert_eq!(format!("{:?}", e1), "Protobuf");

    let e2 = Encoding::SBE;
    assert_eq!(format!("{:?}", e2), "SBE");
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", Encoding::Binary), "Binary");
    assert_eq!(format!("{}", Encoding::Protobuf), "Protobuf");
    assert_eq!(format!("{}", Encoding::SBE), "SBE");
}
