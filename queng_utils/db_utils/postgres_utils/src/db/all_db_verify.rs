use crate::prelude::PostgresUtilError;
use crate::PostgresUtil;

impl PostgresUtil {
    /// Verifies all databases associated with the PostgresUtil object.
    ///
    /// This function verifies the specs database by calling the `verify_spec_db` method on the `specs` object.
    /// It returns `Ok(true)` if all databases are successfully verified, and `Err(PostgresUtilError)` if any verification fails.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether all databases have been verified.
    /// If successful, it returns `Ok(true)`.
    /// If an error occurs, it returns `Err(PostgresUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `PostgresUtilError`.
    ///
    pub async fn verify_all_db(&self) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_all_db");

        match self.specs.verify_spec_db().await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify specs DB: {}",
                    e
                )))
            }
        }

        Ok(true)
    }
}
