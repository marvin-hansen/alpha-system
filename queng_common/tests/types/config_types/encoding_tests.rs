use common::prelude::Encoding;

#[test]
fn test_default() {
    let protocol = Encoding::default();
    assert_eq!(protocol, Encoding::NullVal);
}

#[test]
fn test_from_i32() {
    assert_eq!(Encoding::from(0x1_i32), Encoding::Binary);
    assert_eq!(Encoding::from(0x2_i32), Encoding::Protobuf);
    assert_eq!(Encoding::from(0x3_i32), Encoding::SBE);
    assert_eq!(Encoding::from(0x4_i32), Encoding::NullVal);
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
