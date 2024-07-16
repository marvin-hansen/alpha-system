use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        return Err(PostgresUtilError::from("Not implemented".to_string()));
    }
}
