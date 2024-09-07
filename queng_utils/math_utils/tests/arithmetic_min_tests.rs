use math_utils::arithmetic::min;

#[test]
fn test_min_positive_numbers() {
    let numbers = vec![3.0, 1.0, 4.0, 2.0];
    assert_eq!(min(&numbers), 1.0);
}

// returns the smallest number in a list of positive numbers
#[test]
fn test_min_smallest_number_in_positive_list() {
    let numbers = [3.0, 1.5, 4.2, 2.8];
    let result = min(&numbers);
    assert_eq!(result, 1.5);
}

#[test]
fn test_min_smallest_negative_number() {
    let numbers = [-5.0, -10.0, -3.0, -7.0];
    assert_eq!(min(&numbers), -10.0);
}
#[test]
fn test_min_single_element_list() {
    let numbers = [7.0];
    let result = min(&numbers);
    assert_eq!(result, 7.0);
}

#[test]
fn test_min_empty_list() {
    let numbers: Vec<f64> = vec![];
    let result = std::panic::catch_unwind(|| min(&numbers));
    assert!(result.is_err());
}
