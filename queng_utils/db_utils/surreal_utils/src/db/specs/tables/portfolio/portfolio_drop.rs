use crate::db::Specs;
use crate::prelude::SurrealUtilError;

impl Specs {
    pub async fn drop_portfolio_table(&self) -> Result<(), SurrealUtilError> {
        return Err(SurrealUtilError("Not implemented".to_string()));
    }
}
