mod assets;
mod exchanges;
mod instruments;
// mod symbols;
mod stats;

pub(crate) const DB_NAME: &str = "metadata";

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Metadata {}

impl Metadata {
    pub fn new() -> Self {
        Self {}
    }
}

impl Metadata {
    pub fn create_metadata_db(&self) -> String {
        format!("CREATE DATABASE IF NOT EXISTS {DB_NAME}")
    }

    pub fn drop_metadata_db(&self) -> String {
        format!("DROP DATABASE IF EXISTS {DB_NAME}")
    }
}
