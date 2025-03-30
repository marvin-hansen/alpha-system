use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct MessageProcessorError(pub String);

impl MessageProcessorError {
    #[inline]
    pub const fn new(field0: String) -> Self {
        Self(field0)
    }
}

impl Error for MessageProcessorError {}

impl Display for MessageProcessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
