use common_database::sanitize_utils::sanitize_table_name;
use common_errors::SanitizeError;

#[test]
fn test_sanitize_table_name_empty() {
    let result = sanitize_table_name("");
    assert!(matches!(result, Err(SanitizeError::EmptyTableName(_))));
}

#[test]
fn test_sanitize_table_name_invalid_characters() {
    let result = sanitize_table_name("invalid_table!");
    assert!(matches!(result, Err(SanitizeError::InvalidTableName(_))));
}

#[test]
fn test_sanitize_table_name_too_long() {
    let long_table_name = "a".repeat(65);
    let result = sanitize_table_name(&long_table_name);
    assert!(matches!(result, Err(SanitizeError::TableNameTooLong(_))));
}
