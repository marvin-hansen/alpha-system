pub mod db;
pub mod ddl;
pub mod error;
pub mod fields;
pub mod insert;
pub mod prelude;
pub mod query;
pub mod query_utils;
pub mod types;

use klickhouse::{Client, ClientOptions};

/// Creates a new clickhouse to connect to the provided DSN.
/// DSN format: url:port
pub async fn get_clickhouse_client(dsn: String) -> Client {
    let client = Client::connect(dsn.clone(), ClientOptions::default())
        .await
        .expect(format!("[get_clickhouse_client]: Failed to connect to {}", &dsn).as_str());

    client
}
