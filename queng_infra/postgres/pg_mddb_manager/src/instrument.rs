use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use pg_mddb::prelude::Instrument;

impl PostgresMDDBManager {
    pub async fn count_instruments(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_instruments");
        let conn = &mut self.get_connection();

        match Instrument::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }
}
