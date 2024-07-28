use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VerifyBinaryError {
    BinaryNotFound(String),
    BinaryWrongArchitecture(String, String, String),
    BinaryWrongPlatform(String, String, String),
}

impl Error for VerifyBinaryError {}

impl Display for VerifyBinaryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyBinaryError::BinaryNotFound(e) => {
                write!(f, "[VerifyBinaryError]: Binary not found: {e}")
            }
            VerifyBinaryError::BinaryWrongArchitecture(target, actual, error) => {
                write!(
                    f,
                    "[VerifyBinaryError]: Binary has wrong architecture. \n
                    Binary architecture should be {}, but found {} due to error {}",
                    target, actual, error
                )
            }
            VerifyBinaryError::BinaryWrongPlatform(target, actual, error) => {
                write!(
                    f,
                    "[VerifyBinaryError]: Binary has wrong platform. \n
                    Binary platform should be {}, but found {} due to error {}",
                    target, actual, error
                )
            }
        }
    }
}
