use binary_protocol::Validatable;
use std::fmt;
use stream_errors::ValidationError;

// Define custom error type for testing
#[derive(Debug, PartialEq)]
enum TestError {
    InvalidId,
    IncorrectName,
    UnsupportedLength,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::InvalidId => write!(f, "Invalid ID"),
            TestError::IncorrectName => write!(f, "Incorrect name"),
            TestError::UnsupportedLength => write!(f, "Unsupported length"),
        }
    }
}

impl std::error::Error for TestError {}

// Define test structs
#[derive(Debug)]
struct TestValidatable {
    id: u32,
    name: String,
    data: Vec<u8>,
}

impl TestValidatable {
    fn new(id: u32, name: &str, data: Vec<u8>) -> Self {
        Self {
            id,
            name: name.to_string(),
            data,
        }
    }
}

impl Validatable<TestError> for TestValidatable {
    fn validate(&self) -> Result<(), TestError> {
        // ID must be greater than 0
        if self.id == 0 {
            return Err(TestError::InvalidId);
        }

        // Name must not be empty
        if self.name.is_empty() {
            return Err(TestError::IncorrectName);
        }

        // Data must not be too long
        if self.data.len() > 100 {
            return Err(TestError::UnsupportedLength);
        }

        Ok(())
    }
}

// Define a struct using the library's ValidationError
#[derive(Debug)]
struct LibraryValidatable {
    id: u32,
    payload: Vec<u8>,
}

impl LibraryValidatable {
    fn new(id: u32, payload: Vec<u8>) -> Self {
        Self { id, payload }
    }
}

impl Validatable<ValidationError> for LibraryValidatable {
    fn validate(&self) -> Result<(), ValidationError> {
        // ID must be greater than 0
        if self.id == 0 {
            return Err(ValidationError::ClientIdMustNotBeNull);
        }

        // Payload must not be empty
        if self.payload.is_empty() {
            return Err(ValidationError::EmptyMessagePayload);
        }

        Ok(())
    }
}

#[test]
fn test_validatable_valid() {
    let valid = TestValidatable::new(1, "test", vec![1, 2, 3]);
    assert!(valid.validate().is_ok());
}

#[test]
fn test_validatable_invalid_id() {
    let invalid = TestValidatable::new(0, "test", vec![1, 2, 3]);
    let result = invalid.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), TestError::InvalidId);
}

#[test]
fn test_validatable_invalid_name() {
    let invalid = TestValidatable::new(1, "", vec![1, 2, 3]);
    let result = invalid.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), TestError::IncorrectName);
}

#[test]
fn test_validatable_invalid_length() {
    let invalid = TestValidatable::new(1, "test", vec![0; 101]);
    let result = invalid.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), TestError::UnsupportedLength);
}

#[test]
fn test_library_validatable_valid() {
    let valid = LibraryValidatable::new(1, vec![1, 2, 3]);
    assert!(valid.validate().is_ok());
}

#[test]
fn test_library_validatable_invalid_id() {
    let invalid = LibraryValidatable::new(0, vec![1, 2, 3]);
    let result = invalid.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ValidationError::ClientIdMustNotBeNull
    ));
}

#[test]
fn test_library_validatable_empty_payload() {
    let invalid = LibraryValidatable::new(1, vec![]);
    let result = invalid.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ValidationError::EmptyMessagePayload
    ));
}

#[test]
fn test_validatable_chain() {
    // Test chaining multiple validations
    let valid1 = TestValidatable::new(1, "test1", vec![1, 2, 3]);
    let valid2 = TestValidatable::new(2, "test2", vec![4, 5, 6]);

    let result = valid1.validate().and_then(|_| valid2.validate());
    assert!(result.is_ok());

    let invalid1 = TestValidatable::new(0, "test1", vec![1, 2, 3]);
    let valid2 = TestValidatable::new(2, "test2", vec![4, 5, 6]);

    let result = invalid1.validate().and_then(|_| valid2.validate());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), TestError::InvalidId);
}
