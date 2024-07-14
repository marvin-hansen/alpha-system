use crate::db::Specs;
use crate::prelude::SurrealUtilError;

impl Specs {
    pub(crate) async fn create_service_table(&self) -> Result<(), SurrealUtilError> {
        let ddl = "";

        Ok(())
    }
}
