use crate::db::Specs;
use crate::prelude::SurrealUtilError;

impl Specs {
    pub async fn count_portfolios(&self) -> Result<u64, SurrealUtilError> {
        let count = match self.db.count_portfolio_config().await {
            Ok(res) => res,
            Err(e) => return Err(SurrealUtilError::new(e.to_string())),
        };

        Ok(count)
    }
}
