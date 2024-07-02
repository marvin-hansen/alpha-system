use klickhouse::Client;

mod db;

mod tables;
// pub(crate) const DB_NAME: &'static str = "specs";
//
// pub(crate) const DB_TABLES: [&'static str; 1] = ["services"];

#[derive(Clone)]
pub struct Specs {
    client: Client,
}

impl Specs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
