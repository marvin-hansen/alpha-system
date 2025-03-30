/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::fields::{DEX, NON_TRADE_INSTRUMENT_ID, NON_UNIQUE_EXCHANGE_INSTRUMENT_ID};
use crate::init::InitManager;
use crate::init::patches::INSTRUMENT_PATCHES;
use common_errors::InitError;
use common_metadata::MetaInstrument;

impl InitManager {
    ///
    /// Asynchronously initializes level 3 instruments by downloading and processing instrument data.
    ///
    /// # Returns
    /// Returns a vector of `MetaInstrument` structs on success, or an `InitError` on failure.
    ///
    /// This method prints debug messages based on the use of a proxy and debug mode.
    ///
    pub(super) async fn init_level_3_instruments(
        &self,
        valid_exchanges: &[String],
    ) -> Result<Vec<MetaInstrument>, InitError> {
        self.dbg_print("Level 3: Download reference instrument data!");
        let downloaded_instruments = self
            .dl_utils
            .download_instruments()
            .await
            .expect("Failed to download instrument data");

        if self.use_proxy {
            if self.dbg {
                let msg = format!(
                    "Level 3: Returning {} downloaded Instruments",
                    downloaded_instruments.len()
                );
                self.dbg_print(&msg);
            }
            return Ok(downloaded_instruments);
        }

        self.dbg_print("Level 3: Process the downloaded instrument data");
        let processed_instruments = self
            .process_instruments(&downloaded_instruments, valid_exchanges)
            .await
            .expect("Failed to process reference Instrument data");

        if self.dbg {
            let msg = format!(
                "Level 3: Returning {} valid Instruments",
                processed_instruments.len()
            );
            self.dbg_print(&msg);
        }

        // Free memory.
        drop(downloaded_instruments);

        Ok(processed_instruments)
    }

    async fn process_instruments(
        &self,
        downloaded_instruments: &[MetaInstrument],
        valid_exchanges: &[String],
    ) -> Result<Vec<MetaInstrument>, InitError> {
        // By experience, at least 90% of the reference data are junk (inactive) thus small alloc.
        let capacity = downloaded_instruments.len() * 0.10 as usize;
        let mut processed_instruments = Vec::with_capacity(capacity);

        for i in downloaded_instruments {
            if is_valid_instrument(i, valid_exchanges) {
                //  if not, add instrument to the list
                if !requires_patching(i) {
                    processed_instruments.push(i.to_owned());
                } else {
                    // if it does need patching, swap out the original instrument with the patched one
                    let patched_instrument = self.patch_instruments(i.to_owned());
                    // and then add the patched instrument to the list
                    processed_instruments.push(patched_instrument);
                }
            }
        }

        // Remove duplicates
        processed_instruments.dedup();

        // Sort instruments by exchange code
        processed_instruments.sort_by(|a, b| a.exchange_code.cmp(&b.exchange_code));

        Ok(processed_instruments)
    }
}

fn requires_patching(instrument: &MetaInstrument) -> bool {
    for (exchange, instrument_id, _, _) in INSTRUMENT_PATCHES {
        if instrument.exchange_code.eq(exchange) && instrument.exchange_pair_code.eq(instrument_id)
        {
            return true;
        }
    }

    false
}

// Double check if instrument is inactive i.e. from an inactive exchange
fn is_valid_instrument(instrument: &MetaInstrument, valid_exchanges: &[String]) -> bool {
    // Non-trading instruments
    if instrument.trade_count == 0 {
        return false;
    }

    // Instrument  inactive
    if instrument.trade_start_time.is_none() && instrument.trade_end_time.is_none() {
        return false;
    }

    // Instrument inactive
    if instrument.trade_end_time.is_some() && instrument.trade_end_timestamp.is_some() {
        return false;
    }

    // Instrument of no interest
    if NON_TRADE_INSTRUMENT_ID.contains(&instrument.class.as_str()) {
        return false;
    }

    // Instruments listed on decentralized exchanges (DEX)
    if DEX.contains(&instrument.exchange_code.as_str()) {
        return false;
    }

    // Instruments listed on a non-trading exchange
    if !valid_exchanges.contains(&instrument.exchange_code.to_lowercase()) {
        return false;
    }

    if NON_UNIQUE_EXCHANGE_INSTRUMENT_ID.contains(&instrument.exchange_pair_code.as_str()) {
        return false;
    }

    true
}
