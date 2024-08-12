use crate::model::instrument::{CreateInstrument, Instrument, UpdateInstrument};
use crate::schema::cmdb::instrument::dsl::*;
use crate::Connection;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

impl Instrument {
    pub fn create(db: &mut Connection, item: &CreateInstrument) -> QueryResult<Self> {
        diesel::insert_into(instrument)
            .values(item)
            .returning(Instrument::as_returning())
            .get_result(db)
    }

    pub fn check_if_instrument_code_exists(
        db: &mut Connection,
        param_instrument_code: String,
    ) -> QueryResult<bool> {
        match instrument
            .find(param_instrument_code)
            .first::<Instrument>(db)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn read(db: &mut Connection, param_code: String) -> QueryResult<Self> {
        instrument
            .filter(code.eq(param_code))
            .limit(1)
            .get_result(db)
    }

    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<Self>> {
        instrument.load(db)
    }

    pub fn update(
        db: &mut Connection,
        param_code: String,
        item: &UpdateInstrument,
    ) -> QueryResult<Self> {
        diesel::update(instrument.filter(code.eq(param_code)))
            .set(item)
            .returning(Instrument::as_returning())
            .get_result(db)
    }

    pub fn delete(db: &mut Connection, param_code: String) -> QueryResult<usize> {
        diesel::delete(instrument.filter(code.eq(param_code))).execute(db)
    }
}
