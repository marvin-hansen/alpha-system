use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_portfolio_table");

        let types_ddl = self.generate_portfolio_table_types_ddl();
        self.execute_query(&types_ddl)
            .await
            .expect("Failed to create composite types for portfolio table");

        let ddl = self.generate_portfolio_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
