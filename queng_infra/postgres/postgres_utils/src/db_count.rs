use crate::prelude::PostgresUtilError;
use crate::PostgresUtil;
use pg_cmdb::prelude::portfolio;
use pg_smdb::prelude::service;

impl PostgresUtil {
    /// Counts the number of services in the SMDB database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that contains the number of services in the SMDB database.
    /// If successful, it returns `Ok(u64)`.
    /// If an error occurs, it returns `Err(PostgresUtilError)`.
    ///
    pub async fn count_services(&self) -> Result<u64, PostgresUtilError> {
        let conn = &mut self.pool.get().unwrap();

        self.dbg_print("[count_services]: count all services in SMDB DB schema");

        match service::Service::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }

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
