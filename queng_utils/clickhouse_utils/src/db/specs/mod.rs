#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Specs {}

impl Specs {
    pub fn new() -> Self {
        Self {}
    }
}

impl Specs {
    pub fn create_specs_db(&self) -> String {
        "CREATE DATABASE IF NOT EXISTS specs".to_string()
    }

    pub fn drop_specs_db(&self) -> String {
        "DROP DATABASE IF EXISTS specs".to_string()
    }
}
