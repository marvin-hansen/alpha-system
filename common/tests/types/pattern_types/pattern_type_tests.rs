use common::prelude::PatternType;

#[test]
fn test_debug() {
    let pattern_type = PatternType::Base;
    assert_eq!("Base", format!("{:?}", pattern_type));

    let pattern_type = PatternType::Extra;
    assert_eq!("Extra", format!("{:?}", pattern_type));
}

#[test]
fn test_pattern_type_default_display() {
    let pattern_type = PatternType::Base;
    let expected = "Base";
    let result = format!("{}", pattern_type);
    assert_eq!(result, expected);

    let pattern_type = PatternType::Extra;
    let expected = "Extra";
    let result = format!("{}", pattern_type);
    assert_eq!(result, expected);
}
