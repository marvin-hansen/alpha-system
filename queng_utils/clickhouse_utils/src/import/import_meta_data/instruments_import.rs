use crate::error::ClickHouseUtilError;
use crate::fields::inactive_exchanges::INACTIVE_EXCHANGES;
use crate::types::{Instrument, InstrumentsRoot};
use crate::ClickhouseUtil;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

impl ClickhouseUtil {
    pub async fn import_instruments_data(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let instruments = self
            .load_instruments(path)
            .await
            .expect("Failed to load instrument.json file");

        for instrument in instruments.iter() {
            // Skip all instruments from inactive exchanges
            if INACTIVE_EXCHANGES.contains(&instrument.exchange_code()) {
                continue;
            }

            if self.is_valid_instrument(instrument) {
                let insert_query = self.metadata.generate_instruments_insert(instrument);
                self.execute_query(&insert_query)
                    .await
                    .expect("Failed to insert asset")
            }
        }

        Ok(())
    }

    async fn load_instruments(&self, path: &str) -> Result<Vec<Instrument>, ClickHouseUtilError> {
        let file_path = PathBuf::from(path);
        let file = File::open(file_path).expect("instruments.json file not found");
        let instruments: InstrumentsRoot =
            serde_json::from_reader(file).expect("error while reading");
        Ok(instruments.data)
    }

    // Double check if instrument is inactive i.e. from an inactive exchange
    fn is_valid_instrument(&self, instrument: &Instrument) -> bool {
        // Instrument  inactive
        if instrument.trade_start_time.is_none() && instrument.trade_end_time.is_none() {
            return false;
        }

        // Instrument inactive
        if instrument.trade_end_time.is_some() && instrument.trade_end_timestamp.is_some() {
            return false;
        }

        // Instrument is of no interest
        if instrument.class.eq("option") {
            return false;
        }

        // Non-perpetual future contracts.
        if instrument.class.eq("future") {
            return false;
        }

        true
    }
}
