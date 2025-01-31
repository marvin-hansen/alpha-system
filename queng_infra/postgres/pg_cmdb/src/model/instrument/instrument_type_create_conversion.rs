/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::instrument::CreateInstrument;

use common_exchange::Instrument as CommonInstrument;

impl CreateInstrument {
    #[must_use]
    pub fn from_common_instrument(instrument: &CommonInstrument) -> Self {
        Self {
            code: instrument.code().to_string(),
            class: instrument.class().to_string(),
            exchange_code: instrument.exchange_code().to_string(),
            exchange_pair_code: instrument.exchange_pair_code().to_string(),
            base_asset: instrument.base_asset().to_string(),
            quote_asset: instrument.quote_asset().to_string(),
            instrument_figi: instrument.instrument_figi().clone(),
        }
    }
}
