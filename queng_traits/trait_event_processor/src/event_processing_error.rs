use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct EventProcessingError(pub String);

impl EventProcessingError {
    #[inline]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for EventProcessingError {}

impl fmt::Display for EventProcessingError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventProcessingError: {}", self.0)
    }
}
