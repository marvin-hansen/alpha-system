use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use diesel::{sql_query, RunQueryDsl};

impl Specs {
    pub(crate) async fn execute_query(&mut self, query: String) -> Result<(), PostgresUtilError> {
        let conn = self
            .pool
            .get()
            .await
            .expect("Failed to get a DB connection from the pool");

        conn.interact(move |conn| {
            sql_query(query).execute(conn).expect("Failed query");
        })
        .await
        .expect("");

        Ok(())
    }
}
