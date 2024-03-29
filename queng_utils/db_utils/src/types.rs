use klickhouse::Row;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Row, Serialize, Deserialize)]
pub struct CountRow {
    count: u64,
}

impl CountRow {
    pub fn count(&self) -> u64 {
        self.count
    }
}

impl Display for CountRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.count)
    }
}
