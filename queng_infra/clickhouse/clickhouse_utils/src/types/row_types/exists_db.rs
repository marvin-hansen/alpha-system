use klickhouse::Row;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Row, Serialize, Deserialize)]
pub struct ExistsDBRow {
    // Return type of an exists query is an unsigned u8 integer
    // https://clickhouse.com/docs/en/sql-reference/statements/exists
    value: String,
}

impl ExistsDBRow {
    #[must_use]
    pub const fn value(&self) -> &String {
        &self.value
    }
}

impl Display for ExistsDBRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DB Name: {}", self.value())
    }
}
