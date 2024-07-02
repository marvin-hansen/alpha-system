use klickhouse::Row;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Row, Serialize, Deserialize)]
pub struct ExistsRow {
    // Return type of an exists query is an unsigned u8 integer
    // https://clickhouse.com/docs/en/sql-reference/statements/exists
    value: u8,
}

impl ExistsRow {
    pub fn exists(&self) -> bool {
        return if self.value > 0 { true } else { false };
    }
}

impl Display for ExistsRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table exits: {}", self.exists())
    }
}
