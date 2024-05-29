use crate::errors::InitError;
use crate::init::InitManager;
use common::prelude::{Instrument, SymbolMapping};

impl InitManager {
    // Implement once the algorithm for constructing symbol mapping is ready.
    pub(super) async fn init_level_4_symbol_mapping(
        &self,
        _instruments: &Vec<Instrument>,
    ) -> Result<Vec<SymbolMapping>, InitError> {
        Ok(Vec::default())
    }
}
