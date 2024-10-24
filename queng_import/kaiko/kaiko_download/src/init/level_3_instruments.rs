use crate::fields::DEX;
use crate::init::InitManager;
use common_errors::prelude::InitError;
use common_metadata::prelude::MetaInstrument;

impl InitManager {
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
                self.dbg_print(&msg)
            }
            return Ok(downloaded_instruments);
        }

        self.dbg_print("Level 3: Process the downloaded instrument data");
        let processed_instruments = process_instruments(&downloaded_instruments, valid_exchanges)
            .await
            .expect("Failed to process reference Instrument data");

        if self.dbg {
            let msg = format!(
                "Level 3: Returning {} valid Instruments",
                processed_instruments.len()
            );
            self.dbg_print(&msg)
        }

        // Free memory.
        drop(downloaded_instruments);

        Ok(processed_instruments)
    }
}

async fn process_instruments(
    downloaded_instruments: &[MetaInstrument],
    valid_exchanges: &[String],
) -> Result<Vec<MetaInstrument>, InitError> {
    // By experience, at least 90% of the reference data are junk (inactive) thus small alloc.
    let capacity = downloaded_instruments.len() * 0.10 as usize;
    let mut processed_instruments = Vec::with_capacity(capacity);

    for i in downloaded_instruments.iter() {
        if is_valid_instrument(i, valid_exchanges) {
            processed_instruments.push(i.to_owned())
        }
    }

    Ok(processed_instruments)
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
    if instrument.class.eq("option") {
        return false;
    }

    // Instrument of no interest
    if instrument.class.eq("option_combo") {
        return false;
    }

    // Instrument of no interest
    if instrument.class.eq("future_combo") {
        return false;
    }

    // Non-perpetual future contracts.
    if instrument.class.eq("future") {
        return false;
    }

    // Instruments listed on decentralized exchanges (DEX)
    if DEX.contains(&instrument.exchange_code.as_str()) {
        return false;
    }

    // Instruments listed
    if !valid_exchanges.contains(&instrument.exchange_code.to_lowercase()) {
        return false;
    }

    true
}
