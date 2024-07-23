use crate::common::common_ddl::{ddl_db, ddl_verify};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops a database in the PostgreSQL server.
    ///
    /// This method drops the specified database in the PostgreSQL server.
    /// It performs the following steps:
    ///
    /// 1. Prints a debug message indicating the intention to drop the database.
    /// 2. Generates a DROP DATABASE query using the provided database name.
    /// 3. Executes the DROP DATABASE query using the `execute_query` method.
    /// 4. Returns `Ok(())` if the database is dropped successfully.
    /// 5. Returns an `Err` variant of `PostgresUtilError` if there is an error during the dropping process.
    ///
    /// # Arguments
    ///
    /// * `db_name` - A string slice containing the name of the database to drop.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the database is dropped successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error during the dropping process.
    ///
    pub(crate) async fn drop_db(&self, db_name: &str) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_spec_db");

        let query = &ddl_db::generate_drop_db_ddl(db_name);
        match self.execute_query(query).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(e.to_string()));
            }
        };

        Ok(())
    }

    /// Verifies the existence of a table in the database.
    ///
    /// This method verifies the existence of a table in the database.
    /// It performs the following steps:
    ///
    /// 1. Prints a debug message indicating the verification of the table's existence.
    /// 2. Generates a verification query using the provided schema name and table name.
    /// 3. Executes the verification query using the `execute_verify_query` method.
    /// 4. Returns `Ok(true)` if the table exists, or `Ok(false)` if it does not exist.
    /// 5. Returns an `Err` variant of `PostgresUtilError` if there is an error during the verification process.
    ///
    /// # Arguments
    ///
    /// * `schema_name` - A string slice containing the schema name of the table.
    /// * `table_name` - A string slice containing the name of the table to verify.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the table exists, or `Ok(false)` if it does not exist.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error during the verification process.
    ///
    pub(crate) async fn verify_table_exists(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_tables_exists");

        let query = &ddl_verify::generate_verify_table_ddl(schema_name, table_name);
        match self.execute_verify_query(query).await {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(e.to_string()));
            }
        };

        Ok(true)
    }
}

impl Specs {
    /// Executes a query in the database.
    ///
    /// This method executes the provided query in the database.
    /// It performs the following steps:
    ///
    /// 1. Prints a debug message indicating the execution of the query.
    /// 2. Executes the provided query using the database connection.
    /// 3. Checks the execution result.
    /// 4. Returns `Ok(())` if the query is executed successfully, or an `Err` variant of `PostgresUtilError` if there is an error.
    ///
    /// # Arguments
    ///
    /// * `query` - A string slice containing the query to be executed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the query is executed successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error during the query execution.
    ///
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), PostgresUtilError> {
        self.dbg_print("execute_query");

        match self.db.query(query, &[]).await {
            Ok(_) => Ok(()),
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresUtilError::new(e.to_string()))
            }
        }
    }

    /// Executes a verification query in the database and returns the result as a boolean.
    ///
    /// This method executes a verification query in the database and returns the result as a boolean.
    /// It performs the following steps:
    ///
    /// 1. Prints a debug message indicating the execution of the verification query.
    /// 2. Executes the provided query using `query_one` method of the database connection.
    /// 3. Retrieves the result from the query and converts it to a boolean.
    /// 4. Returns the boolean result.
    ///
    /// If the execution is successful, it returns the boolean result of the query. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Arguments
    ///
    /// * `query` - A string slice containing the verification query to be executed.
    ///
    /// # Returns
    ///
    /// Returns the boolean result of the query if the query is executed successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error during the query execution.
    ///
    pub(crate) async fn execute_verify_query(
        &self,
        query: &str,
    ) -> Result<bool, PostgresUtilError> {
        self.dbg_print("execute_verify_query");

        match self.db.query_one(query, &[]).await {
            Ok(row) => {
                let exists = row.get::<usize, bool>(0);

                Ok(exists)
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresUtilError::new(e.to_string()))
            }
        }
    }

    pub(crate) async fn execute_insert_query(&self, query: &str) -> Result<u64, PostgresUtilError> {
        self.dbg_print("execute_insert_query");

        match self.db.query_one(query, &[]).await {
            Ok(row) => {
                let id = row.get::<usize, i64>(0);

                Ok(id as u64)
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresUtilError::new(e.to_string()))
            }
        }
    }

    /// Executes a count query in the database and returns the count as a `u64`.
    ///
    /// This method executes a count query in the database and returns the count as a `u64`.
    /// It performs the following steps:
    ///
    /// 1. Prints a debug message indicating the execution of the count query.
    /// 2. Executes the provided query using `query_one` method of the database connection.
    /// 3. Retrieves the count as an `i64`.
    /// 4. Converts the count to a `u64` and returns it.
    ///
    /// If the execution is successful, it returns the count as a `u64`.
    /// Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Arguments
    ///
    /// * `query` - A string slice containing the count query to be executed.
    ///
    /// # Returns
    ///
    /// Returns the count as a `u64` if the query is executed successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error during the query execution.
    ///
    pub(crate) async fn execute_count_query(&self, query: &str) -> Result<u64, PostgresUtilError> {
        self.dbg_print("execute_count_query");

        let row = self.db.query_one(query, &[]).await;
        match row {
            Ok(row) => {
                let count = row.get::<usize, i64>(0);
                Ok(count as u64)
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresUtilError::new(e.to_string()))
            }
        }
    }
}
