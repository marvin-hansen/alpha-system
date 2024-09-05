use crate::model::instrument::{Instrument, UpdateInstrument};
use crate::schema::mddb::instruments::table as instruments_table;
use crate::Connection;
use common_metadata::prelude::MetaInstrument;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};

impl Instrument {
    pub fn create(
        conn: &mut Connection,
        meta_instrument: MetaInstrument,
    ) -> Result<MetaInstrument, diesel::result::Error> {
        let instrument = Instrument::from_meta_instrument(meta_instrument.clone());
        match diesel::insert_into(instruments_table)
            .values(&instrument)
            .get_result::<Instrument>(conn)
        {
            Ok(res) => Ok(res.to_meta_instrument()),
            Err(e) => Err(e),
        }
    }

    pub fn create_instrument_collection(
        conn: &mut Connection,
        meta_instruments: Vec<MetaInstrument>,
    ) -> Result<bool, diesel::result::Error> {
        let instruments: Vec<Instrument> = meta_instruments
            .iter()
            .map(|meta_instrument| Instrument::from_meta_instrument(meta_instrument.clone()))
            .collect();
        diesel::insert_into(instruments_table)
            .values(&instruments)
            .execute(conn)
            .map(|_| true)
    }

    pub fn count(conn: &mut Connection) -> Result<u64, diesel::result::Error> {
        instruments_table
            .count()
            .get_result::<i64>(conn)
            .map(|c| c as u64)
    }

    pub fn check_if_instrument_id_exists(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> QueryResult<bool> {
        let exists = instruments_table
            .find(instrument_id)
            .first::<Instrument>(conn)
            .optional()?
            .is_some();
        Ok(exists)
    }

    pub fn read(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> Result<MetaInstrument, diesel::result::Error> {
        instruments_table
            .filter(crate::schema::mddb::instruments::instrument_id.eq(instrument_id))
            .first::<Instrument>(conn)
            .map(|instrument| instrument.to_meta_instrument())
    }

    pub fn read_all(conn: &mut Connection) -> Result<Vec<MetaInstrument>, diesel::result::Error> {
        instruments_table
            .load::<Instrument>(conn)
            .map(|instruments| {
                instruments
                    .iter()
                    .map(|instrument| instrument.to_meta_instrument())
                    .collect()
            })
    }

    pub fn update(
        conn: &mut Connection,
        instrument_id: &str,
        meta_instrument: MetaInstrument,
    ) -> Result<usize, diesel::result::Error> {
        let update_instrument = UpdateInstrument::from_meta_instrument(meta_instrument);
        diesel::update(
            instruments_table
                .filter(crate::schema::mddb::instruments::instrument_id.eq(instrument_id)),
        )
        .set(&update_instrument)
        .execute(conn)
    }

    pub fn delete(
        conn: &mut Connection,
        instrument_id: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(
            instruments_table
                .filter(crate::schema::mddb::instruments::instrument_id.eq(instrument_id)),
        )
        .execute(conn)
    }
}
