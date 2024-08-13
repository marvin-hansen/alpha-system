use crate::model::instrument::Instrument;

use common_exchange::prelude::Instrument as CommonInstrument;

impl Instrument {
    pub fn from_common_instrument(instrument: &CommonInstrument) -> Instrument {
        Instrument {
            code: instrument.code().to_string(),
            class: instrument.class().to_string(),
            exchange_code: instrument.exchange_code().to_string(),
            exchange_pair_code: instrument.exchange_pair_code().to_string(),
            base_asset: instrument.base_asset().to_string(),
            quote_asset: instrument.quote_asset().to_string(),
            instrument_figi: instrument.instrument_figi().clone().map(|x| x.to_string()),
        }
    }

    pub fn to_common_instrument(&self) -> CommonInstrument {
        CommonInstrument::new(
            self.code.clone(),
            self.class.clone(),
            self.exchange_code.clone(),
            self.exchange_pair_code.clone(),
            self.base_asset.clone(),
            self.quote_asset.clone(),
            self.instrument_figi.clone().map(|x| x.to_string()),
        )
    }
}
