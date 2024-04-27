use crate::error::QueryError;
use crate::types::CountRow;
use crate::ClickhouseUtil;
use common::prelude::ValidationError;
use klickhouse::{Client, KlickhouseError};

impl ClickhouseUtil {
    /// Executes a query on the specified table in the ClickHouse database.
    pub async fn execute_query(&self, query: &str) -> Result<(), QueryError> {
        // execute query
        let res = self.client.execute(query).await;

        // check for errors
        return match res {
            Ok(_) => Ok(()),
            Err(e) => Err(QueryError::QueryFailed(e.to_string())),
        };
    }

    /// Counts the number of rows in the specified table in the ClickHouse database.
    pub async fn count_rows(&self, table_name: &str) -> Result<u64, QueryError> {
        // Generate count query for the specified table
        let count_query = format!("SELECT count(*) FROM {table_name}");

        // We need type annotation of the Result type here.
        let number_of_rows: Result<CountRow, KlickhouseError> =
            self.client.query_one(&count_query).await;

        return match number_of_rows {
            Ok(number_of_rows) => Ok(number_of_rows.count()),
            Err(e) => Err(QueryError::QueryFailed(e.to_string())),
        };
    }
}
