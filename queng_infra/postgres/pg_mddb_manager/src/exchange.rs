use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use pg_mddb::prelude::Exchange;

impl PostgresMDDBManager {
    pub async fn count_exchanges(&mut self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_exchanges");
        let conn = &mut self.get_connection();

        match Exchange::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }
}
