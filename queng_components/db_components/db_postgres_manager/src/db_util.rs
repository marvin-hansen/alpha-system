use crate::error::PostgresDBError;
use crate::PostgresDBManager;

impl PostgresDBManager {
    /// Executes a query to check if a row exists in the database.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the query executes without error, returns `Ok(true)` if a row exists, `Ok(false)` otherwise.
    /// If the query fails, returns a `PostgresDBError`.
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
    /// Executes a query on the Postgres database.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    ///
    /// # Returns
    ///
    /// * `Result<(), PostgresDBError>` - A result indicating success or failure.
    /// If the query executes without error, returns `Ok(())`.
    /// If the query fails, returns a `PostgresDBError`.
    ///
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

    /// Executes an insert query on the Postgres database.
    ///
    /// # Arguments
    ///
    /// * `query` - The insert query to execute.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    /// If the query executes without error, returns the inserted row's primary key as a `u64`.
    /// If the query fails, returns a `PostgresDBError`.
    ///
    pub(crate) async fn execute_insert_query(&self, query: &str) -> Result<u64, PostgresDBError> {
        self.dbg_print("execute_insert_query");

        match self.client.query_one(query, &[]).await {
            Ok(row) => {
                // PG Sequence primary key seems to be int4, which maps to i32 in Rust
                // https://gist.github.com/steveh/7c7145409a5eed6b698ee8b609b6d1fc
                let id = row.get::<usize, i32>(0);

                Ok(id as u64)
            }
            Err(e) => {
                self.dbg_print(&format!("Insert Query failed: \n {}", query));
                Err(PostgresDBError::InsertFailed(e.to_string()))
            }
        }
    }

    /// Executes a query to count the number of rows in a table.
    ///
    /// # Arguments
    ///
    /// * `schema_name` - The name of the schema containing the table.
    /// * `table_name` - The name of the table to count rows in.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    /// If successful, returns the count of rows in the table as a `u64`.
    ///
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
