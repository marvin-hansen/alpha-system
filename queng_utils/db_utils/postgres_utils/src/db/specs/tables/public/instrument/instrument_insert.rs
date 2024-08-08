use common_exchange::prelude::Instrument;
use common_pg_queries::portfolio_insert;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Inserts a new instrument into the `public.instrument` table.
    ///
    /// # Arguments
    ///
    /// * `data` - The instrument to be inserted.
    ///
    /// # Returns
    ///
    /// Returns the ID of the inserted instrument on success.
    ///
    /// # Errors
    ///
    /// Returns a `PostgresUtilError` if the insertion fails.
    ///
    pub(crate) async fn insert_instrument(
        &self,
        data: &Instrument,
    ) -> Result<String, PostgresUtilError> {
        self.dbg_print("insert_instrument");

        let query = portfolio_insert::build_insert_instrument_query(data);
        // println!("query: {}", query);
        match self.db.query_one(&query, &[]).await {
            Ok(row) => {
                let code = row.get::<usize, String>(0);
                Ok(code)
            }
            Err(err) => Err(PostgresUtilError::new(format!(
                "Failed to insert instrument: {} due error: {}",
                &data.code(),
                err
            ))),
        }
    }
}
