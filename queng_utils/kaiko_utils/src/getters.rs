use crate::error::KaikoUtilError;
use crate::inactive_exchanges::INACTIVE_EXCHANGES;
use crate::KaikoUtil;
use common::prelude::{Asset, Exchange, Instrument};

impl KaikoUtil {
    pub async fn get_assets(&self) -> Result<Vec<Asset>, KaikoUtilError> {
        self.dbg_print("[get_assets]: Download asset metadata from Kaiko.");
        return match self.client.get_assets().await {
            Ok(assets) => Ok(assets.data),
            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving assets: {}",
                e.to_string()
            ))),
        };
    }

    pub async fn get_exchanges(&self) -> Result<Vec<Exchange>, KaikoUtilError> {
        self.dbg_print("[get_exchanges]: Download exchange metadata from Kaiko.");
        let exchanges = match self.client.get_exchanges().await {
            Ok(exchanges) => exchanges.data,
            Err(e) => {
                return Err(KaikoUtilError::new(&format!(
                    "Error retrieving exchanges: {}",
                    e.to_string()
                )));
            }
        };

        // Create a new vector with only active exchanges
        let size = exchanges.len() - INACTIVE_EXCHANGES.len() + 1;
        let mut res: Vec<Exchange> = Vec::with_capacity(size);

        self.dbg_print("[get_exchanges]: Remove inactive exchanges.");
        for exchange in exchanges {
            // Only add active exchanges to the return vector
            if !INACTIVE_EXCHANGES.contains(&exchange.name.as_str()) {
                res.push(exchange);
            }
        }

        return Ok(res);
    }

    pub async fn get_instruments(&self) -> Result<Vec<Instrument>, KaikoUtilError> {
        self.dbg_print("[get_instruments]: Download instrument metadata from Kaiko.");

        let instruments = match self.client.get_instruments().await {
            Ok(instruments) => instruments.data,
            Err(e) => {
                return Err(KaikoUtilError::new(&format!(
                    "Error retrieving instruments: {}",
                    e.to_string()
                )));
            }
        };

        // Create a new vector with only active instruments
        // By experience, almost 90% of instruments are
        // inactive so setting the size to 20% should suffice.
        let size = instruments.len() * 0.20 as usize;
        let mut res: Vec<Instrument> = Vec::with_capacity(size);

        self.dbg_print("[get_instruments]: Remove inactive instruments.");
        for instrument in instruments {
            if !INACTIVE_EXCHANGES.contains(&instrument.exchange_code.as_str()) {
                // Only add active instruments to the return vector
                if self.is_valid_instrument(&instrument) {
                    res.push(instrument);
                }
            }
        }

        return Ok(res);
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
