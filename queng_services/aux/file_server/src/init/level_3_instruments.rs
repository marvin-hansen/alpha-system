use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::Instrument;

impl InitManager {
    pub(super) async fn init_level_3_instruments(&self) -> Result<Vec<Instrument>, InitError> {
        self.dbg_print("Level 3: Download reference instrument data!");
        let downloaded_instruments = utils::download_instruments()
            .await
            .expect("Failed to download instrument data");

        self.dbg_print("Level 3: Process the downloaded instrument data");
        let processed_instruments = self
            .process_instruments(downloaded_instruments)
            .await
            .expect("Failed to process reference Instrument data");

        if self.dbg {
            let msg = format!(
                "Level 3: Returning {} valid Instruments",
                processed_instruments.len()
            );
            self.dbg_print(&msg)
        }

        Ok(processed_instruments)
    }

    async fn process_instruments(
        &self,
        downloaded_instruments: Vec<Instrument>,
    ) -> Result<Vec<Instrument>, InitError> {
        // By experience, at least 90% of the reference data are junk ie inactive thus small alloc.
        let capacity = downloaded_instruments.len() * 0.10 as usize;
        let mut processed_instruments = Vec::with_capacity(capacity);

        for i in downloaded_instruments.iter() {
            if is_valid_instrument(i) {
                processed_instruments.push(i.to_owned())
            }
        }

        Ok(processed_instruments)
    }
}

// Double check if instrument is inactive i.e. from an inactive exchange
fn is_valid_instrument(instrument: &Instrument) -> bool {
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

    // Non-perpetual future contracts.
    if instrument.class.eq("future") {
        return false;
    }

    true
}
