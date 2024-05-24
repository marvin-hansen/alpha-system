use crate::errors::InitError;
use crate::init::InitManager;
use crate::utils;
use common::prelude::{Exchange, Instrument};

impl InitManager {
    pub(super) async fn init_level_3_instruments(
        &self,
        valid_exchanges: &Vec<Exchange>,
    ) -> Result<Vec<Instrument>, InitError> {
        // Download the instruments data
        self.download_instruments()
            .await
            .expect("Failed to download reference Instrument data");

        // Load the instruments data from the downloaded file
        let downloaded_instruments = self
            .load_instruments()
            .await
            .expect("Failed to download reference Instrument data");

        // Process the downloaded instruments
        let valid_instruments = self
            .process_instruments(valid_exchanges, &downloaded_instruments)
            .await
            .expect("Failed to process reference Instrument data");

        Ok(valid_instruments)
    }

    async fn download_instruments(&self) -> Result<(), InitError> {
        utils::download_instruments()
            .await
            .expect("Failed to download instrument data");

        Ok(())
    }

    async fn load_instruments(&self) -> Result<Vec<Instrument>, InitError> {
        let instruments = utils::load_instruments()
            .await
            .expect("Failed to load instruments from download file");

        Ok(instruments)
    }

    async fn process_instruments(
        &self,
        _valid_exchanges: &Vec<Exchange>,
        _downloaded_instruments: &Vec<Instrument>,
    ) -> Result<Vec<Instrument>, InitError> {
        Ok(Vec::new())
    }
}
