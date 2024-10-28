mod instrument_impl;
mod instrument_queries;
mod instrument_type_conversion;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName, Selectable};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Queryable,
    QueryableByName,
    Selectable,
    Identifiable,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name=crate::schema::mddb::instruments)]
#[diesel(primary_key(instrument_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Instrument {
    pub instrument_id: String,
    pub instrument_hash: String,
    pub instrument_code: String,
    pub instrument_class: String,
    pub instrument_base_asset: String,
    pub instrument_quote_asset: String,
    pub instrument_exchanges_code: String,
    pub instrument_exchange_pair_code: String,
    pub instrument_pair_figi: Option<String>,
    pub instrument_figi: Option<String>,
    pub instrument_trade_start_timestamp: Option<i64>,
    pub instrument_trade_end_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Queryable, QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::instruments)]
#[diesel(primary_key(instrument_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateInstrument {
    pub instrument_id: String,
    pub instrument_code: String,
    pub instrument_hash: String,
    pub instrument_class: String,
    pub instrument_base_asset: String,
    pub instrument_quote_asset: String,
    pub instrument_exchanges_code: String,
    pub instrument_exchange_pair_code: String,
    pub instrument_pair_figi: Option<String>,
    pub instrument_figi: Option<String>,
    pub instrument_trade_start_timestamp: Option<i64>,
    pub instrument_trade_end_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Queryable, QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::instruments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateInstrument {
    pub instrument_class: String,
    pub instrument_code: String,
    pub instrument_hash: String,
    pub instrument_base_asset: String,
    pub instrument_quote_asset: String,
    pub instrument_exchanges_code: String,
    pub instrument_exchange_pair_code: String,
    pub instrument_pair_figi: Option<String>,
    pub instrument_figi: Option<String>,
    pub instrument_trade_start_timestamp: Option<i64>,
    pub instrument_trade_end_timestamp: Option<i64>,
}
