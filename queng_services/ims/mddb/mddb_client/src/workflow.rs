use crate::error::SymdbClientError;
use crate::MDMClient;

const FN_NAME: &str = "[MDMClient]: ";

impl MDMClient {}

/// Creates a SymdbClientError with a formatted error message.
///
/// # Arguments
///
/// * `msg` - The message describing what failed.
/// * `err` - The underlying error message.
///
/// # Returns
///
/// Returns a SymdbClientError struct containing the formatted error message.
///
fn get_error(msg: &str, err: &str) -> SymdbClientError {
    SymdbClientError(format!("{} {} because of Error {}", FN_NAME, msg, err))
}
