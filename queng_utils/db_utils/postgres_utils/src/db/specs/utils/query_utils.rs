use crate::db::common_ddl::{ddl_db, ddl_table};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub(crate) async fn drop_db(&self, db_name: &str) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_spec_db");
        let drop_ddl = &ddl_db::generate_drop_db_ddl(db_name);
        match self.execute_query(drop_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError(format!(
                    "Failed to drop specs DB: {}",
                    e.to_string()
                )))
            }
        };

        Ok(())
    }

    pub(crate) async fn verify_table_exists(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<bool, PostgresUtilError> {
        self.dbg_print("verify_tables_exists");
        let verify_ddl = &ddl_table::generate_verify_table_ddl(schema_name, table_name);

        match self.execute_verify_query(verify_ddl, table_name).await {
            Ok(res) => {
                if !res {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to verify system schema: {}",
                    e.to_string()
                )))
            }
        };

        Ok(true)
    }
}

impl Specs {
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), PostgresUtilError> {
        self.dbg_print("execute_query");
        self.dbg_print(query);

        match self.db.query(query, &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }

    pub(crate) async fn execute_verify_query(
        &self,
        query: &str,
        target: &str,
    ) -> Result<bool, PostgresUtilError> {
        self.dbg_print("execute_verify_query");
        self.dbg_print(query);

        return match self.db.query_one(query, &[]).await {
            Ok(row) => {
                let target_row = row.get::<usize, String>(0);

                self.dbg_print(&format!("db_row: {}", target_row));
                self.dbg_print(&format!("target: {}", target));

                if target_row.to_lowercase().eq(&target.to_lowercase()) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        };
    }

    pub(crate) async fn execute_count_query(&self, query: &str) -> Result<u64, PostgresUtilError> {
        self.dbg_print("execute_count_query");
        self.dbg_print(query);

        let row = self.db.query_one(query, &[]).await;
        match row {
            Ok(row) => {
                let count = row.get::<usize, i64>(0);
                Ok(count as u64)
            }
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
