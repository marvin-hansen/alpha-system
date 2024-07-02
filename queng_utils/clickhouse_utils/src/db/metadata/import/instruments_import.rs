use crate::db::metadata::Metadata;
use common::prelude::Instrument;
use std::error::Error;

impl Metadata {
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
