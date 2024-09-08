use crate::prelude::PostgresUtil;
use crate::prelude::PostgresUtilError;
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
}
