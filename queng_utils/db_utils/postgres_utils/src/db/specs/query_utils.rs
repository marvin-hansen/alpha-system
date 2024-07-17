use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), PostgresUtilError> {
        self.dbg_print("execute_query");

        match self.db.query(query, &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }

    pub(crate) async fn execute_verify_query(
        &self,
        query: &str,
        target: &str,
    ) -> Result<bool, PostgresUtilError> {
        self.dbg_print("execute_verify_query");

        return match self.db.query_one(query, &[]).await {
            Ok(row) => {
                let target_row = row.get::<usize, String>(0);

                self.dbg_print(&format!("db_row: {}", target_row));
                self.dbg_print(&format!("target: {}", target));

                if target_row.to_lowercase().eq(&target.to_lowercase()) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        };
    }
}
