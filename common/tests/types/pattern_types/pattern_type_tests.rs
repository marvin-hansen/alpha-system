use common::prelude::PatternType;

#[test]
fn pattern_type_base_displays_as_base() {
    let pattern_type = PatternType::Base;
    assert_eq!("Base", format!("{:?}", pattern_type));
}

#[test]
fn pattern_type_extra_displays_as_extra() {
    let pattern_type = PatternType::Extra;
    assert_eq!("Extra", format!("{:?}", pattern_type));
}

#[test]
fn test_pattern_type_default_display() {
    let pattern_type = PatternType::default();
    let expected = "Base";
    let result = format!("{:?}", pattern_type);
    assert_eq!(result, expected);
}
