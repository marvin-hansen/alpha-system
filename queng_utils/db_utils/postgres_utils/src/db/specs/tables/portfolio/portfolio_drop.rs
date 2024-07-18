use crate::db::utils::ddl;
use crate::db::{Specs, PORTFOLIO_TABLE};
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_portfolio_table");
        let ddl = &ddl::generate_drop_table_ddl(PORTFOLIO_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
