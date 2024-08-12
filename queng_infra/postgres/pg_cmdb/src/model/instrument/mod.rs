use crate::schema::cmdb::instrument;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};

mod instrument_impl;

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=instrument, primary_key(code))]
pub struct Instrument {
    pub code: String,
    pub class: String,
    pub exchange_code: String,
    pub exchange_pair_code: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub instrument_figi: Option<String>,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=instrument)]
pub struct CreateInstrument {
    pub code: String,
    pub class: String,
    pub exchange_code: String,
    pub exchange_pair_code: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub instrument_figi: Option<String>,
}

impl CreateInstrument {
    pub fn new(
        code: String,
        class: String,
        exchange_code: String,
        exchange_pair_code: String,
        base_asset: String,
        quote_asset: String,
        instrument_figi: Option<String>,
    ) -> Self {
        Self {
            code,
            class,
            exchange_code,
            exchange_pair_code,
            base_asset,
            quote_asset,
            instrument_figi,
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=instrument)]
pub struct UpdateInstrument {
    pub class: Option<String>,
    pub exchange_code: Option<String>,
    pub exchange_pair_code: Option<String>,
    pub base_asset: Option<String>,
    pub quote_asset: Option<String>,
    pub instrument_figi: Option<Option<String>>,
}

impl UpdateInstrument {
    pub fn new(
        class: Option<String>,
        exchange_code: Option<String>,
        exchange_pair_code: Option<String>,
        base_asset: Option<String>,
        quote_asset: Option<String>,
        instrument_figi: Option<Option<String>>,
    ) -> Self {
        Self {
            class,
            exchange_code,
            exchange_pair_code,
            base_asset,
            quote_asset,
            instrument_figi,
        }
    }
}
