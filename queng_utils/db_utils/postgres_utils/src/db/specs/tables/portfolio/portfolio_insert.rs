use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_exchange::prelude::PortfolioConfig;

impl Specs {
    pub async fn insert_portfolio(&self, _data: &PortfolioConfig) -> Result<(), SurrealUtilError> {
        return Err(SurrealUtilError::from("Not implemented".to_string()));
    }
}
