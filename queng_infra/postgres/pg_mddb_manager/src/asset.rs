use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use pg_mddb::prelude::Asset;

impl PostgresMDDBManager {
    pub async fn count_assets(&mut self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_assets");
        let conn = &mut self.get_connection();

        match Asset::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }
}
