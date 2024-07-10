use crate::db::specs::{Specs, DB_NAME};
use std::error::Error;

impl Specs {
    /// Asynchronously drops the ClickHouse database specified in the `DB_NAME` constant if it exists.
    ///
    /// This method generates a Data Definition Language (DDL) query to drop the database with the name specified in the `DB_NAME` constant.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of dropping the database.
    /// If the database is dropped successfully or does not exist, it returns `Ok(())`.
    /// If an error occurs during the dropping process, it returns `Err(Box<dyn Error>)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `Error` trait.
    ///
    pub async fn drop_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = format!("DROP DATABASE IF EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }
}
