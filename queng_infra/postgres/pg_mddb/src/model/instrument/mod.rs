mod instrument_type_conversion;

use bon::builder;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

#[builder]
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::instruments)]
#[diesel(primary_key(instrument_id))]
pub struct Instrument {
    pub instrument_id: String,
    pub instrument_class: String,
    pub instrument_base_asset: String,
    pub instrument_quote_asset: String,
    pub instrument_exchanges_code: String,
    pub instrument_exchange_pair_code: String,
    pub instrument_trade_start_timestamp: Option<i64>,
    pub instrument_trade_end_timestamp: Option<i64>,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::instruments)]
#[diesel(primary_key(instrument_id))]
pub struct CreateInstrument {
    pub instrument_id: String,
    pub instrument_class: String,
    pub instrument_base_asset: String,
    pub instrument_quote_asset: String,
    pub instrument_exchanges_code: String,
    pub instrument_exchange_pair_code: String,
    pub instrument_trade_start_timestamp: Option<i64>,
    pub instrument_trade_end_timestamp: Option<i64>,
}

#[builder]
#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=crate::schema::mddb::instruments)]
pub struct UpdateInstrument {
    pub instrument_class: String,
    pub instrument_base_asset: String,
    pub instrument_quote_asset: String,
    pub instrument_exchanges_code: String,
    pub instrument_exchange_pair_code: String,
    pub instrument_trade_start_timestamp: Option<i64>,
    pub instrument_trade_end_timestamp: Option<i64>,
}
