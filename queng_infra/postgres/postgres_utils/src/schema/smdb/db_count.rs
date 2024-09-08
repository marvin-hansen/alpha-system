use crate::prelude::PostgresUtil;
use crate::prelude::PostgresUtilError;
use pg_cmdb::prelude::portfolio;

impl PostgresUtil {
    /// Counts the number of portfolios in the CMDB database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that contains the number of portfolios in the CMDB database.
    /// If successful, it returns `Ok(u64)`.
    /// If an error occurs, it returns `Err(PostgresUtilError)`.
    ///
    pub async fn count_portfolios(&self) -> Result<u64, PostgresUtilError> {
        let conn = &mut self.pool.get().unwrap();
        self.dbg_print("[count_portfolios]: count all portfolios in CMDB DB schema");

        match portfolio::Portfolio::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
