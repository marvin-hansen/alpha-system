use crate::model::instrument::UpdateInstrument;
use common_exchange::Instrument as CommonInstrument;

impl UpdateInstrument {
    #[must_use]
    pub fn from_common_instrument(instrument: &CommonInstrument) -> Self {
        Self {
            class: Some(instrument.class().to_string()),
            exchange_code: Some(instrument.exchange_code().to_string()),
            exchange_pair_code: Some(instrument.exchange_pair_code().to_string()),
            base_asset: Some(instrument.base_asset().to_string()),
            quote_asset: Some(instrument.quote_asset().to_string()),
            instrument_figi: Some(instrument.instrument_figi().clone()),
        }
    }
}
