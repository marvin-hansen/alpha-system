use crate::db::specs::Specs;
use crate::query_utils;
use std::error::Error;

impl Specs {
    pub async fn drop_spec_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = self.drop_specs_ddl();
        query_utils::execute_query(&self.client, &ddl)
            .await
            .expect("Failed to drop specs DB");

        Ok(())
    }

    fn drop_specs_ddl(&self) -> String {
        "DROP DATABASE IF EXISTS specs".to_string()
    }
}
