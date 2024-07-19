use crate::db::all_db_constants::DB_NAME;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_spec_db(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_spec_db");
        match self.drop_db(DB_NAME).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to drop specs DB: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
