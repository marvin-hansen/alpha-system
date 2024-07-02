use crate::error::QueryError;
use crate::query_utils;
use klickhouse::Client;

// mod symbols;
mod import;
mod info;
mod setup;
mod tables;
mod teardown;

pub(crate) const DB_NAME: &'static str = "metadata";
pub(crate) const DB_TABLES: [&'static str; 4] = ["assets", "exchanges", "instruments", "stats"];

#[derive(Clone)]
pub struct Metadata {
    client: Client,
}

impl Metadata {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Metadata {
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), QueryError> {
        query_utils::execute_query(&self.client, &query)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }
    pub(crate) async fn verify_table_exists(&self, query: &str) -> Result<bool, QueryError> {
        let res = query_utils::verify_table_exists(&self.client, &query)
            .await
            .expect("Failed to drop specs DB");

        Ok(res)
    }
}
