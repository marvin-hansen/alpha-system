use common_errors::SanitizeError;

/// Sanitizes the provided table name to prevent SQL injection attacks.
///
/// # Arguments
///
/// * `table_name` - The table name to sanitize
///
/// # Returns
///
/// A `Result` containing the original table name if valid, or a `SanitizeError`
/// if the name is invalid.
///
/// # Errors
///
/// - `SanitizeError::EmptyTableName` if `table_name` is empty
/// - `SanitizeError::InvalidTableName` if `table_name` contains invalid characters
/// - `SanitizeError::TableNameTooLong` if `table_name` is longer than 64 characters
///
///
/// This checks `table_name` for:
///
/// - Emptiness
/// - Invalid characters
/// - Length less than 64 characters
///
/// If valid, it returns the original `table_name`.
pub fn sanitize_table_name(table_name: &str) -> Result<&str, SanitizeError> {
    // check for empty name
    if table_name.is_empty() {
        return Err(SanitizeError::EmptyTableName(format!(
            "Table: {}",
            table_name
        )));
    }

    // check for invalid characters
    if table_name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Err(SanitizeError::InvalidTableName(format!(
            "Table: {}",
            table_name
        )));
    }

    // check for length
    if table_name.len() > 64 {
        return Err(SanitizeError::TableNameTooLong(format!(
            "Table: {}",
            table_name
        )));
    }

    Ok(table_name)
}
