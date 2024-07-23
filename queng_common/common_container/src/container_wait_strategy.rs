use std::fmt::{Display, Formatter};

/// An enum that represents the wait strategy for a container.
///
#[derive(Debug, Default, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
pub enum WaitStrategy {
    #[default]
    Never,
    WaitForDuration(u64),
    WaitUntilConsoleOutputContains(String),
}

impl Display for WaitStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
