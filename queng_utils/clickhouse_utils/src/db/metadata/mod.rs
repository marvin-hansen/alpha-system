mod ddl;
mod inserts;

pub(crate) const TABLE_NAME: &str = "metadata";

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Metadata {}

impl Metadata {
    pub fn new() -> Self {
        Self {}
    }
}

impl Metadata {
    pub fn create_metadata_db(&self) -> String {
        format!("CREATE DATABASE IF NOT EXISTS {TABLE_NAME}")
    }

    pub fn drop_metadata_db(&self) -> String {
        format!("DROP DATABASE IF EXISTS {TABLE_NAME}")
    }
}
