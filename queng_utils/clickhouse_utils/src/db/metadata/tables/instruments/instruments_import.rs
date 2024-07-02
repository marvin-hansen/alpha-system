use crate::db::metadata::Metadata;
use common::prelude::Instrument;
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
