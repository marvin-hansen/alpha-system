use crate::error::PostgresDBError;
use crate::PostgresDBManager;

impl PostgresDBManager {
    pub(crate) async fn execute_exists_query(&self, query: &str) -> Result<bool, PostgresDBError> {
        self.dbg_print("execute_exists_query");

        match self.client.query_one(query, &[]).await {
            Ok(row) => {
                let exists = row.get::<usize, bool>(0);
                Ok(exists)
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresDBError::QueryFailed(e.to_string()))
            }
        }
    }
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), PostgresDBError> {
        self.dbg_print("execute_query");

        match self.client.query(query, &[]).await {
            Ok(_) => Ok(()),
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresDBError::QueryFailed(e.to_string()))
            }
        }
    }
    pub(crate) async fn execute_count_query(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<u64, PostgresDBError> {
        self.dbg_print("execute_count_query");

        let query = format!("SELECT COUNT(*) FROM {schema_name}.{table_name};");

        let row = self.client.query_one(&query, &[]).await;
        match row {
            Ok(row) => {
                let count = row.get::<usize, i64>(0);
                Ok(count as u64)
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresDBError::CountFailed(e.to_string()))
            }
        }
    }
}
