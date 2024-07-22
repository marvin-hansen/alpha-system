use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_portfolio_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_portfolio_table");

        self.dbg_print("create_portfolio_table/account_type");
        let types_ddl = self.generate_portfolio_table_account_type_ddl();
        match self.execute_query(&types_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create portfolio table types: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_portfolio_table");
        let ddl = self.generate_portfolio_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create portfolio table: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_portfolio_table/index");
        let index_ddl = self.generate_portfolio_table_index_ddl();
        match self.execute_query(&index_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create portfolio table indexes: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
