use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(crate::model::exchange::Exchange))]
#[diesel(belongs_to(crate::model::instrument::Instrument))]
#[diesel(table_name=crate::schema::mddb::instruments_exchanges)]
#[diesel(primary_key(instrument_id, exchange_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InstrumentsExchanges {
    pub instrument_id: String,
    pub exchange_id: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=crate::schema::mddb::instruments_exchanges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateInstrumentsExchanges {
    pub instrument_id: String,
    pub exchange_id: String,
}
