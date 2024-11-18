use math_utils::abs;

#[test]
fn test_abs_negative_input() {
    assert_eq!(abs(-5.0), 5.0);
}

// abs function returns positive value for positive input
#[test]
fn test_abs_positive_input() {
    let result = abs(5.0);
    assert_eq!(result, 5.0);
}

#[test]
fn test_abs_large_positive_input() {
    let result = abs(1e308);
    assert_eq!(result, 1e308);
}
