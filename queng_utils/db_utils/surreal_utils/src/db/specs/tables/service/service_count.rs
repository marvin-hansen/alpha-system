use crate::db::Specs;
use crate::prelude::SurrealUtilError;

impl Specs {
    pub async fn count_services(&self) -> Result<u64, SurrealUtilError> {
        let res = match self.db.read_all_services().await {
            Ok(res) => res,
            Err(e) => return Err(SurrealUtilError::new(e.to_string())),
        };

        let count = res.len() as u64;

        Ok(count)
    }
}
