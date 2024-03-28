use klickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct CountRow {
    count: u64,
}

impl CountRow {
    pub fn count(&self) -> u64 {
        self.count
    }
}
