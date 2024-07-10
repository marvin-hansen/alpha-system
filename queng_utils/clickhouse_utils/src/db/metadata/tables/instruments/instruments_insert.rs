use crate::db::metadata::{Metadata, DB_NAME, INSTRUMENTS_TABLE};
use common::prelude::{Instrument, InstrumentMetadata};
use std::error::Error;

impl Metadata {
    /// Imports a vector of instruments metadata into the metadata database.
    ///
    /// This method takes a vector of `Instrument` objects and imports their metadata into the metadata database.
    /// It generates an insert query for each instrument using the `generate_instruments_insert` method and executes it using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `instruments` - A reference to a vector of `Instrument` objects containing the metadata to be imported.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - A result indicating the success of the import operation. Returns `Ok(())` if the import is successful, or an `Err` containing the error if it fails.
    ///
    pub async fn import_instruments_metadata(
        &self,
        instruments: &Vec<Instrument>,
    ) -> Result<(), Box<dyn Error>> {
        for instrument in instruments.iter() {
            let insert_query = self.generate_instruments_insert(instrument);

            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }
}

impl Metadata {
    /// Generates an SQL insert query for inserting instrument metadata into the instruments table.
    ///
    /// This method takes an `Instrument` object and generates an SQL insert query for inserting its metadata into the instruments table.
    ///
    /// # Arguments
    ///
    /// * `instrument` - A reference to the `Instrument` object for which the insert query is generated.
    ///
    /// # Returns
    ///
    /// * `String` - The SQL insert query as a string.
    ///
    pub(crate) fn generate_instruments_insert(&self, instrument: &Instrument) -> String {
        let table_name = format!("{DB_NAME}.{INSTRUMENTS_TABLE}");
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
