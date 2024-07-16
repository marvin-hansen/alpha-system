use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        return Err(PostgresUtilError("Not implemented".to_string()));
    }
}
