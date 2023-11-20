use common::types::exchange_types::security_type::SecurityType;

#[test]
fn test_default_variant() {
    let security_type: SecurityType = Default::default();
    assert_eq!(security_type, SecurityType::Spot);
}

#[test]
fn test_unknown_security_type() {
    let security_type: SecurityType = SecurityType::UnknownSecurityType;
    assert_eq!(security_type, SecurityType::UnknownSecurityType);
}

#[test]
fn test_display_spot() {
    assert_eq!(format!("{:?}", SecurityType::Spot), "Spot");
}

#[test]
fn test_display_index() {
    assert_eq!(format!("{:?}", SecurityType::Index), "Index");
}

#[test]
fn test_display_future() {
    assert_eq!(format!("{:?}", SecurityType::Future), "Future");
}

#[test]
fn test_display_perpetual_future() {
    assert_eq!(
        format!("{:?}", SecurityType::PerpetualFuture),
        "PerpetualFuture"
    );
}

#[test]
fn test_display_option() {
    assert_eq!(format!("{:?}", SecurityType::Option), "Option");
}

#[test]
fn test_display_future_option() {
    assert_eq!(format!("{:?}", SecurityType::FutureOption), "FutureOption");
}
