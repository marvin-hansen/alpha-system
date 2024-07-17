use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub(crate) async fn execute_query(&mut self, query: &str) -> Result<(), PostgresUtilError> {
        match self.db.query(query, &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }

    pub(crate) async fn execute_verify_query(
        &mut self,
        query: &str,
        target: &str,
    ) -> Result<bool, PostgresUtilError> {
        return match self.db.query_one(query, &[]).await {
            Ok(row) => {
                let target_row = row.get::<usize, String>(0);
                if target_row.contains(target) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        };
    }
}
