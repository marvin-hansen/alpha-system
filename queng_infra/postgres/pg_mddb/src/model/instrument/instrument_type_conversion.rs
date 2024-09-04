use crate::model::instrument::Instrument;
use common_metadata::prelude::MetaInstrument;

impl Instrument {
    pub fn from_meta_instrument(meta_instrument: MetaInstrument) -> Self {
        Instrument {
            instrument_id: meta_instrument.code,
            instrument_class: meta_instrument.class,
            instrument_base_asset: meta_instrument.base_asset,
            instrument_quote_asset: meta_instrument.quote_asset,
            instrument_exchanges_code: meta_instrument.exchange_code,
            instrument_exchange_pair_code: meta_instrument.exchange_pair_code,
            instrument_trade_start_timestamp: meta_instrument
                .trade_start_timestamp
                .map(|ts| ts as i64),
            instrument_trade_end_timestamp: meta_instrument.trade_end_timestamp,
        }
    }

    pub fn to_meta_instrument(&self) -> MetaInstrument {
        MetaInstrument {
            kaiko_legacy_exchange_slug: String::new(), // Assuming kaiko_legacy_exchange_slug is not used
            trade_start_time: None,                    // Assuming trade_start_time is not used
            trade_end_time: None,                      // Assuming trade_end_time is not used
            exchange_code: self.instrument_exchanges_code.clone(),
            exchange_pair_code: self.instrument_exchange_pair_code.clone(),
            base_asset: self.instrument_base_asset.clone(),
            quote_asset: self.instrument_quote_asset.clone(),
            kaiko_legacy_symbol: String::new(), // Assuming kaiko_legacy_symbol is not used
            code: self.instrument_id.clone(),
            class: self.instrument_class.clone(),
            metadata: None, // Assuming metadata is not used
            trade_start_timestamp: self.instrument_trade_start_timestamp.map(|ts| ts as u64),
            trade_end_timestamp: self.instrument_trade_end_timestamp,
            trade_compressed_size: 0, // Assuming trade_compressed_size is not used
            trade_count: 0,           // Assuming trade_count is not used
        }
    }
}
