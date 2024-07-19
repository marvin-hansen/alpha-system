use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use common_exchange::prelude::PortfolioConfig;

impl Specs {
    pub async fn insert_portfolio(&self, _data: &PortfolioConfig) -> Result<(), PostgresUtilError> {
        Err(PostgresUtilError::from("Not implemented".to_string()))
    }
}
