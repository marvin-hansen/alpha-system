use std::fmt::Debug;

pub mod print_utils;
pub mod shutdown_utils;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ServiceUtil {
    dbg: bool,
}

impl ServiceUtil {
    pub fn new() -> Self {
        Self::build(false)
    }

    pub fn with_debug() -> Self {
        Self::build(true)
    }

    fn build(dbg: bool) -> Self {
        Self { dbg }
    }
}

impl Default for ServiceUtil {
    fn default() -> Self {
        Self::new()
    }
}
