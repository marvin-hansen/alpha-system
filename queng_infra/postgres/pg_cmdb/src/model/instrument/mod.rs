use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

mod instrument_impl;
mod instrument_type_conversion;
mod instrument_type_create_conversion;
mod instrument_type_update_conversion;

#[derive(
    Debug, Clone, PartialEq, Eq, Queryable, Selectable, Identifiable, Insertable, AsChangeset,
)]
#[diesel(table_name=crate::schema::cmdb::instrument, primary_key(code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[diesel(table_name=crate::schema::cmdb::instrument, primary_key(code))]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
    #[must_use]
    pub const fn new(
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
#[diesel(table_name=crate::schema::cmdb::instrument)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateInstrument {
    pub class: Option<String>,
    pub exchange_code: Option<String>,
    pub exchange_pair_code: Option<String>,
    pub base_asset: Option<String>,
    pub quote_asset: Option<String>,
    pub instrument_figi: Option<Option<String>>,
}

impl UpdateInstrument {
    #[must_use]
    pub const fn new(
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
