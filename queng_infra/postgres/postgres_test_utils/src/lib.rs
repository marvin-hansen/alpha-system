mod check;
pub mod prelude;
pub(crate) mod setup;
pub(crate) mod teardown;
pub(crate) mod utils;

use diesel::{Connection, PgConnection};

pub const DB_TEST_URL: &str = "postgres://postgres:postgres@localhost/postgres";

pub async fn postgres_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
