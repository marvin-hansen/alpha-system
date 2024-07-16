use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use diesel::{sql_query, RunQueryDsl};

impl Specs {
    pub(crate) async fn execute_query(&mut self, query: String) -> Result<(), PostgresUtilError> {
        //
        let conn = match self.pool.get().await {
            Ok(conn) => conn,
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        match conn
            .interact(move |conn| {
                sql_query(query).execute(conn).expect("Failed query");
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
