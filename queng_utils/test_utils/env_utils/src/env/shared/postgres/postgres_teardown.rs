use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    /// Teardowns the Postgres database for the environment.
    ///
    /// This method tears down the Postgres database for the environment by performing the following steps:
    ///
    /// 1. Retrieves a new `PostgresUtil` object asynchronously.
    ///
    /// 2. Calls the `teardown_all_db` method of the `PostgresUtil` object with `true` as the argument.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the Postgres database is torn down successfully,
    /// or an `Err` variant of `EnvironmentError` if an error occurs during the teardown process.
    ///
    /// # Errors
    ///
    /// Returns an `EnvironmentError` if any of the following errors occur during the teardown process:
    ///
    /// - If there is an error retrieving the Postgres utility.
    /// - If there is an error tearing down the Postgres database.
    ///
    pub async fn teardown_postgres(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("teardown_postgres");

        self.dbg_print("[teardown_postgres]: Get postgres util");
        let pg_util = &self
            .get_new_postgres_util()
            .await
            .expect("[teardown_postgres]: Failed to get PostgresUtil");

        self.dbg_print("[teardown_postgres]: Teardown postgres database");
        match pg_util.teardown_all_db().await {
            Ok(_) => Ok(()),
            Err(err) => Err(EnvironmentError::new(format!(
                "[teardown_postgres]: Failed to teardown databases: {err}"
            ))),
        }
    }
}
