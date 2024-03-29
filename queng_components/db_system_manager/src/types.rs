use klickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct TestRow {
    id: u32,
}
