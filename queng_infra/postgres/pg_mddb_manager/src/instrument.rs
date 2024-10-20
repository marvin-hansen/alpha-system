use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use common_metadata::prelude::MetaInstrument;
use pg_mddb::prelude::Instrument;

impl PostgresMDDBManager {
    pub async fn insert_instrument(
        &self,
        instrument: MetaInstrument,
    ) -> Result<MetaInstrument, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::create_instrument(conn, instrument) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn insert_instrument_collection(
        &self,
        instruments: &[MetaInstrument],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_instruments_collection");
        let conn = &mut self.get_connection();

        match Instrument::create_instrument_collection(conn, instruments) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn count_instruments(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_instruments");
        let conn = &mut self.get_connection();

        match Instrument::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    pub async fn check_if_instrument_id_exists(
        &self,
        instrument_id: &str,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_instrument_id_exists");
        let conn = &mut self.get_connection();

        match Instrument::check_if_instrument_id_exists(conn, instrument_id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    pub async fn read_instrument(
        &self,
        instrument_id: String,
    ) -> Result<MetaInstrument, PostgresDBError> {
        self.dbg_print("read_instrument");
        let conn = &mut self.get_connection();

        match Instrument::read(conn, &instrument_id) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments(&self) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("read_all_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all(conn) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn update_instrument(
        &self,
        instrument_id: &str,
        instrument: MetaInstrument,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_instrument");
        let conn = &mut self.get_connection();

        match Instrument::update(conn, &instrument_id, instrument) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    pub async fn delete_instrument(&self, instrument_id: String) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_instrument");
        let conn = &mut self.get_connection();

        match Instrument::delete(conn, &instrument_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
