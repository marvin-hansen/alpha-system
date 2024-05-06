use crate::ClickhouseUtil;
use common::prelude::Instrument;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn import_instruments_metadata(
        &self,
        instruments: &Vec<Instrument>,
    ) -> Result<(), Box<dyn Error>> {
        for instrument in instruments.iter() {
            let insert_query = self.metadata.generate_instruments_insert(instrument);

            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }
}
