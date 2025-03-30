use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ValidationError {
    UnknownValidationError = 0,
    EmptyMessagePayload = 1,
    EmptyMessagesCount = 3,
    TooBigMessagePayload(u32, u32) = 4,
    TooLongString(String, usize, usize) = 5,
    PayloadChecksumVerificationFailed = 6,
    PayloadTooLong = 7,
    PayloadExceedsMaximumSize = 8,
    PayloadSizeMismatch = 9,
    ClientIdMustNotBeNull = 10,
}

impl From<u8> for ValidationError {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => ValidationError::UnknownValidationError,
            1 => ValidationError::EmptyMessagePayload,
            3 => ValidationError::EmptyMessagesCount,
            4 => ValidationError::TooBigMessagePayload(0, 0),
            5 => ValidationError::TooLongString(String::new(), 0, 0),
            6 => ValidationError::PayloadChecksumVerificationFailed,
            7 => ValidationError::PayloadTooLong,
            8 => ValidationError::PayloadExceedsMaximumSize,
            9 => ValidationError::PayloadSizeMismatch,
            10 => ValidationError::ClientIdMustNotBeNull,
            _ => ValidationError::UnknownValidationError,
        }
    }
}

impl From<ValidationError> for u8 {
    #[inline]
    fn from(value: ValidationError) -> u8 {
        match value {
            ValidationError::UnknownValidationError => 0,
            ValidationError::EmptyMessagePayload => 1,
            ValidationError::EmptyMessagesCount => 3,
            ValidationError::TooBigMessagePayload(_, _) => 4,
            ValidationError::TooLongString(_, _, _) => 5,
            ValidationError::PayloadChecksumVerificationFailed => 6,
            ValidationError::PayloadTooLong => 7,
            ValidationError::PayloadExceedsMaximumSize => 8,
            ValidationError::PayloadSizeMismatch => 9,
            ValidationError::ClientIdMustNotBeNull => 10,
        }
    }
}

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UnknownValidationError => write!(f, "UnknownValidationError"),
            ValidationError::EmptyMessagePayload => write!(f, "EmptyMessagePayload"),
            ValidationError::EmptyMessagesCount => write!(f, "EmptyMessagesCount"),
            ValidationError::TooBigMessagePayload(size, max) => {
                write!(
                    f,
                    "TooBigMessagePayload: {size} Maximum allowed size: {max}"
                )
            }
            ValidationError::TooLongString(field, size, max) => {
                write!(
                    f,
                    "TooLongString: {field} has size: {size}. Maximum allowed size: {max}"
                )
            }
            ValidationError::PayloadChecksumVerificationFailed => {
                write!(f, "PayloadChecksumVerificationFailed")
            }
            ValidationError::PayloadTooLong => {
                write!(f, "PayloadTooLong")
            }
            ValidationError::PayloadExceedsMaximumSize => {
                write!(f, "PayloadExceedsMaximumSize: Payload exceeds maximum size")
            }
            ValidationError::PayloadSizeMismatch => {
                write!(
                    f,
                    "PayloadSizeMismatch: Payload size does not matches allocated size"
                )
            }
            ValidationError::ClientIdMustNotBeNull => {
                write!(f, "ClientIdMustNotBeNull: ClientId must not be null")
            }
        }
    }
}
