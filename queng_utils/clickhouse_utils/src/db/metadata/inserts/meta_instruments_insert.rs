use crate::db::metadata::{Metadata, TABLE_NAME};
use crate::types::{Instrument, InstrumentMetadata};

impl Metadata {
    pub fn generate_instruments_insert(&self, instrument: &Instrument) -> String {
        let table_name = format!("{TABLE_NAME}.instruments");
        let trade_start_timestamp = instrument.trade_start_timestamp.unwrap_or(0);
        let trade_end_timestamp = instrument.trade_end_timestamp.unwrap_or(0);
        let exchange_code = instrument.exchange_code();
        let exchange_pair_code = &instrument.exchange_pair_code;
        let base_asset = &instrument.base_asset;
        let quote_asset = &instrument.quote_asset;
        let code = &instrument.code;
        let class = &instrument.class;
        let (pair_figi, instrument_figi) = self.extract_instrument_figi(&instrument.metadata);

        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        {trade_start_timestamp},
        {trade_end_timestamp},
        '{exchange_code}',
        '{exchange_pair_code}',
        '{base_asset}',
        '{quote_asset}',
        '{code}',
        '{class}',
        '{pair_figi}',
        '{instrument_figi}'
        )"
        )
        .to_string()
    }

    fn extract_instrument_figi(&self, metadata: &Option<InstrumentMetadata>) -> (String, String) {
        let pair_figi = match metadata {
            Some(metadata) => match &metadata.pair_figi {
                Some(figi) => figi.to_owned(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let instrument_figi = match metadata {
            Some(metadata) => match &metadata.instrument_figi {
                Some(figi) => figi.to_owned(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        (pair_figi, instrument_figi)
    }
}
