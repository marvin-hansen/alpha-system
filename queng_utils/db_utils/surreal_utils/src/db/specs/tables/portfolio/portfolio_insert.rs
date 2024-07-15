use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_exchange::prelude::PortfolioConfig;

impl Specs {
    pub async fn insert_portfolio(&self, data: &PortfolioConfig) -> Result<(), SurrealUtilError> {
        match self.db.insert_portfolio_config(data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealUtilError::from(e.to_string())),
        }
    }
}
