mod common;
mod db;
mod db_setup_revert;
mod errors;
pub mod prelude;
mod types;

use tokio::task::JoinHandle;
use tokio_postgres::NoTls;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub struct PostgresUtil {
    dbg: bool,
    handle: JoinHandle<()>,
    pub specs: Specs,
    pool: Pool<ConnectionManager<PgConnection>>,
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
            println!("[PostgresUtil]: Connecting to Postgres database: {}", dsn);
        }

        let (db, connection) = tokio_postgres::connect(dsn, NoTls)
            .await
            .expect("[PostgresUtil]: Failed to connect to Postgres database");

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        let handle = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!(
                    "[PostgresUtil]: Tokio/Postgres failed to spwan connection task: {}",
                    e
                );
            }
        });

        let specs = Specs::new(dbg, db);

        let pool = postgres_connection_pool(dsn)
            .await
            .expect("[PostgresUtil]: Failed to connect to Postgres database");

        Ok(Self {
            dbg,
            handle,
            specs,
            pool,
        })
    }
}

async fn postgres_connection_pool(
    dsn: &str,
) -> Result<Pool<ConnectionManager<PgConnection>>, PostgresUtilError> {
    let manager = ConnectionManager::<PgConnection>::new(dsn);
    match Pool::builder().test_on_check_out(true).build(manager) {
        Ok(pool) => Ok(pool),
        Err(e) => Err(PostgresUtilError::new(format!(
            "[PostgresUtil]: Failed to build connection pool: {}",
            e
        ))),
    }
}

impl PostgresUtil {
    pub async fn close(&self) {
        self.dbg_print("Closing Postgres connection via Tokio task handle");
        // https://stackoverflow.com/questions/67160923/how-can-you-close-a-tokio-postgres-connection
        self.handle.abort();
    }
}

impl PostgresUtil {
    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresUtil]: {}", msg);
        }
    }
}
