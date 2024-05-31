use crate::errors::InitError;
use crate::init::InitManager;
use common::prelude::{Instrument, InstrumentMapping, SymbolMapping};
use std::collections::{BTreeMap, HashMap, HashSet};

impl InitManager {
    // Implement once the algorithm for constructing symbol mapping is ready.
    pub(super) async fn init_level_4_symbol_mapping(
        &self,
        instruments: &Vec<Instrument>,
    ) -> Result<BTreeMap<String, SymbolMapping>, InitError> {
        let mut symbol_codes = HashSet::new();

        for i in instruments.iter() {
            symbol_codes.insert(i.code.clone());
        }

        // Symbol code <==> Symbol Mapping
        let mut data: BTreeMap<String, SymbolMapping> = BTreeMap::new();

        // Sort symbols
        let mut symbols: Vec<String> = symbol_codes.into_iter().collect();
        symbols.sort();

        for s in symbols.iter() {
            let mut mapping: HashMap<String, InstrumentMapping> = HashMap::new();
            let mut instrument = Instrument::default();

            for i in instruments.iter() {
                if s.eq_ignore_ascii_case(&i.code) {
                    let im = InstrumentMapping::new(
                        i.exchange_code.to_owned(),
                        i.exchange_pair_code.to_owned(),
                        i.clone().metadata.unwrap_or_default().instrument_figi,
                        i.trade_count,
                    );

                    instrument = i.clone();
                    mapping.insert(i.exchange_code.to_owned(), im);
                }
            }

            let sm = SymbolMapping::new(
                instrument.code.to_owned(),
                instrument.class.to_owned(),
                instrument.clone().metadata.unwrap_or_default().pair_figi,
                instrument.base_asset.to_owned(),
                instrument.quote_asset.to_owned(),
                mapping,
            );

            data.insert(s.to_owned(), sm);
        }

        Ok(data)
    }
}
