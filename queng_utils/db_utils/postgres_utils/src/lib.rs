mod db;
mod errors;
pub mod prelude;
mod query_utils;
mod types;

use tokio::task::JoinHandle;
use tokio_postgres::NoTls;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;

pub struct PostgresUtil {
    handle: JoinHandle<()>,
    pub specs: Specs,
}

impl PostgresUtil {
    pub async fn new(dsn: &str) -> Result<Self, PostgresUtilError> {
        Self::build(false, dsn).await
    }

    pub async fn with_debug(dsn: &str) -> Result<Self, PostgresUtilError> {
        Self::build(true, dsn).await
    }

    async fn build(dbg: bool, dsn: &str) -> Result<Self, PostgresUtilError> {
        if dbg {
            println!("[PostgresUtil]: Debug mode enabled");
        }

        let (db, connection) = tokio_postgres::connect(dsn, NoTls)
            .await
            .expect("Failed to connect to Postgres database");

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        let handle = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Postgres connection error: {}", e);
            }
        });

        let specs = Specs::new(dbg, db);

        Ok(Self { handle, specs })
    }
}

impl PostgresUtil {
    pub async fn close(&self) {
        // https://stackoverflow.com/questions/67160923/how-can-you-close-a-tokio-postgres-connection
        self.handle.abort();
    }
}
