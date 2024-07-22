use crate::db::all_db_constants::PORTFOLIO_INSTRUMENT_TABLE;
use crate::db::common_ddl::ddl_table;
use crate::db::Specs;

impl Specs {
    pub async fn drop_portfolio_instrument_table(&self) -> Result<(), String> {
        self.dbg_print("drop_portfolio_instrument_table");

        let ddl = &ddl_table::generate_drop_table_ddl(PORTFOLIO_INSTRUMENT_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                "drop_portfolio_instrument_table: Failed to drop portfolio_instrument table: {}",
                e
            ))
            }
        };

        Ok(())
    }
}
