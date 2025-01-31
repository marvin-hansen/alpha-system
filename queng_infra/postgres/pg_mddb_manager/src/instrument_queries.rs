/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::PostgresMDDBManager;
use common_errors::PostgresDBError;
use common_metadata::MetaInstrument;
use pg_mddb::Instrument;

impl PostgresMDDBManager {
    pub async fn read_instruments_by_exchange_pair_code(
        &self,
        param_instrument_code: &str,
    ) -> Result<Option<MetaInstrument>, PostgresDBError> {
        self.dbg_print("read_instruments_by_exchange_pair_code");
        let conn = &mut self.get_connection();

        match Instrument::read_instruments_by_exchange_pair_code(conn, param_instrument_code) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_instrument_by_figi(
        &self,
        param_figi: &str,
    ) -> Result<Option<MetaInstrument>, PostgresDBError> {
        self.dbg_print("read_instrument_by_figi");
        let conn = &mut self.get_connection();

        match Instrument::read_instrument_by_figi(conn, param_figi) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_instrument_by_pair_figi(
        &self,
        param_pair_figi: &str,
    ) -> Result<Option<MetaInstrument>, PostgresDBError> {
        self.dbg_print("read_instrument_by_pair_figi");
        let conn = &mut self.get_connection();

        match Instrument::read_instrument_by_pair_figi(conn, param_pair_figi) {
            Ok(instrument) => Ok(instrument),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments_for_exchange(
        &self,
        param_exchange: &str,
    ) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all_instruments_for_exchange(conn, param_exchange) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments_for_base_asset(
        &self,
        param_base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all_instruments_for_base_asset(conn, param_base_asset) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments_for_quote_asset(
        &self,
        param_quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all_instruments_for_quote_asset(conn, param_quote_asset) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments_for_base_asset_on_exchange(
        &self,
        param_base_asset: &str,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all_instruments_for_base_asset_on_exchange(
            conn,
            param_base_asset,
            param_exchange_code,
        ) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments_for_quote_asset_on_exchange(
        &self,
        param_quote_asset: &str,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all_instruments_for_quote_asset_on_exchange(
            conn,
            param_quote_asset,
            param_exchange_code,
        ) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_instruments_for_base_quote_asset_on_exchange(
        &self,
        param_base_asset: &str,
        param_quote_asset: &str,
        param_exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, PostgresDBError> {
        self.dbg_print("insert_instruments");
        let conn = &mut self.get_connection();

        match Instrument::read_all_instruments_for_base_quote_asset_on_exchange(
            conn,
            param_base_asset,
            param_quote_asset,
            param_exchange_code,
        ) {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }
}
