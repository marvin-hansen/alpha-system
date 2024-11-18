use math_utils::max;

#[test]
fn test_max_empty_list() {
    let numbers: Vec<f64> = vec![];
    let result = std::panic::catch_unwind(|| max(&numbers));
    assert!(result.is_err());
}

#[test]
fn test_max_from_positive_numbers() {
    let numbers = [1.0, 3.5, 2.2, 4.8, 3.3];
    let result = max(&numbers);
    assert_eq!(result, 4.8);
}

#[test]
fn test_returns_max_negative_numbers() {
    let numbers = [-5.0, -10.0, -3.0, -7.0];
    assert_eq!(max(&numbers), -3.0);
}

#[test]
fn test_returns_max_mixed_numbers() {
    let numbers = [-5.0, 10.0, -3.0, 7.0];
    assert_eq!(max(&numbers), 10.0);
}
