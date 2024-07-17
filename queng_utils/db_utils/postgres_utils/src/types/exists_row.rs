use postgres_types::{FromSql, ToSql};
use std::fmt::{Display, Formatter};

#[derive(Debug, ToSql, FromSql)]
pub struct DatName {
    datname: String,
}

impl DatName {
    pub fn value(&self) -> &String {
        &self.datname
    }
}

impl Display for DatName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NamedSingleRow: {}", self.value())
    }
}
