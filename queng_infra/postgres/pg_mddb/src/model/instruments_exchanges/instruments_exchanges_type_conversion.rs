use crate::prelude::{CreateInstrumentsExchanges, Instrument};
use common_metadata::prelude::MetaInstrument;

impl CreateInstrumentsExchanges {
    pub fn from_meta_instrument(meta_instrument: MetaInstrument) -> Self {
        CreateInstrumentsExchanges {
            instrument_id: meta_instrument.code,
            exchange_id: meta_instrument.exchange_code,
        }
    }

    pub fn from_instrument(meta_instrument: Instrument) -> Self {
        CreateInstrumentsExchanges {
            instrument_id: meta_instrument.instrument_id,
            exchange_id: meta_instrument.instrument_exchanges_code,
        }
    }
}
