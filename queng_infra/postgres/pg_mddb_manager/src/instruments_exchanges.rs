use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use pg_mddb::prelude::InstrumentsExchanges;

impl PostgresMDDBManager {
    pub async fn insert_instruments_exchanges(
        &self,
        instruments_exchanges: InstrumentsExchanges,
    ) -> Result<InstrumentsExchanges, PostgresDBError> {
        self.dbg_print("insert_instruments_exchanges");
        let conn = &mut self.get_connection();

        match InstrumentsExchanges::create(
            conn,
            instruments_exchanges.instrument_id,
            instruments_exchanges.exchange_id,
        ) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn check_if_instruments_exchange_exists(
        &self,
        instrument_id: &str,
        exchange_id: &str,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_instruments_exchange_exists");
        let conn = &mut self.get_connection();

        match InstrumentsExchanges::check_if_exists(
            conn,
            instrument_id.to_string(),
            exchange_id.to_string(),
        ) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    pub async fn delete_instruments_exchanges(
        &self,
        instrument_id: &str,
        exchange_id: &str,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_instruments_exchanges");
        let conn = &mut self.get_connection();

        match InstrumentsExchanges::delete(conn, instrument_id.to_string(), exchange_id.to_string())
        {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
