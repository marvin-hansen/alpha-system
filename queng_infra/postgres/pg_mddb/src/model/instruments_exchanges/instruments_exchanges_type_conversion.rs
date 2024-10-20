use crate::prelude::CreateInstrumentsExchanges;
use common_metadata::prelude::MetaInstrument;

impl CreateInstrumentsExchanges {
    pub fn from_meta_instrument(meta_instrument: MetaInstrument) -> Self {
        CreateInstrumentsExchanges {
            instrument_id: meta_instrument.code,
            exchange_id: meta_instrument.exchange_code,
        }
    }
}
