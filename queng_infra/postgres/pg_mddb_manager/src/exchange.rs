use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use common_metadata::prelude::MetaExchange;
use pg_mddb::prelude::Exchange;

impl PostgresMDDBManager {
    pub async fn insert_exchange(
        &self,
        exchange: MetaExchange,
    ) -> Result<MetaExchange, PostgresDBError> {
        self.dbg_print("insert_exchange");
        let conn = &mut self.get_connection();

        match Exchange::create(conn, exchange) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn insert_exchange_collection(
        &self,
        exchanges: &[MetaExchange],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_exchange_collection");
        let conn = &mut self.get_connection();

        match Exchange::create_exchange_collection(conn, exchanges) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn count_exchanges(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_exchanges");
        let conn = &mut self.get_connection();

        match Exchange::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    pub async fn check_if_exchange_id_exists(
        &self,
        exchange_id_str: String,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_exchange_id_exists");
        let conn = &mut self.get_connection();

        match Exchange::check_if_exchange_id_exists(conn, exchange_id_str) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    pub async fn read_exchange(
        &self,
        exchange_id_str: String,
    ) -> Result<MetaExchange, PostgresDBError> {
        self.dbg_print("read_exchange");
        let conn = &mut self.get_connection();

        match Exchange::read(conn, exchange_id_str) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_exchanges(&self) -> Result<Vec<MetaExchange>, PostgresDBError> {
        self.dbg_print("read_all_exchanges");
        let conn = &mut self.get_connection();

        match Exchange::read_all(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn update_exchange(
        &self,
        exchange_id_str: String,
        exchange: MetaExchange,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_exchange");
        let conn = &mut self.get_connection();

        match Exchange::update(conn, exchange_id_str, exchange) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    pub async fn delete_exchange(&self, exchange_id_str: String) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_exchange");
        let conn = &mut self.get_connection();

        match Exchange::delete(conn, exchange_id_str) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
