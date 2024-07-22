use crate::error::KaikoUtilError;
use crate::KaikoUtil;
use common_metadata::prelude::{MetaAsset, MetaExchange, MetaInstrument, Stats};

impl KaikoUtil {
    /// Retrieves asset metadata from the Kaiko API.
    ///
    /// This method downloads asset metadata from the Kaiko API using the `KaikoClient` instance.
    ///
    /// # Returns
    ///
    /// It returns a `Result` containing a `Vec` of `MetaAsset` structs on success,
    /// or a `KaikoUtilError` on failure.
    ///
    /// # Errors
    ///
    /// If there is an error retrieving the assets from the Kaiko API, a `KaikoUtilError` is returned.
    /// The error message contains the specific error encountered.
    ///
    pub async fn get_assets(&self) -> Result<Vec<MetaAsset>, KaikoUtilError> {
        self.dbg_print("[get_assets]: Download asset metadata from Kaiko.");
        match self.client.get_assets().await {
            Ok(assets) => Ok(assets),
            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving assets: {}",
                e
            ))),
        }
    }

    /// Retrieves exchange metadata from the Kaiko API.
    ///
    /// This method downloads exchange metadata from the Kaiko API using the `KaikoClient` instance.
    ///
    /// # Returns
    ///
    /// It returns a `Result` containing a `Vec` of `MetaExchange` structs on success,
    /// or a `KaikoUtilError` on failure.
    ///
    /// # Errors
    ///
    /// If there is an error retrieving the exchanges from the Kaiko API, a `KaikoUtilError` is returned.
    /// The error message contains the specific error encountered.
    ///
    pub async fn get_exchanges(&self) -> Result<Vec<MetaExchange>, KaikoUtilError> {
        self.dbg_print("[get_exchanges]: Download exchange metadata from Kaiko.");
        match self.client.get_exchanges().await {
            Ok(exchanges) => Ok(exchanges),
            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving exchanges: {}",
                e
            ))),
        }
    }

    /// Retrieves metadata statistics from the Kaiko API.
    ///
    /// This method downloads metadata statistics from the Kaiko API using the `KaikoClient` instance.
    ///
    /// # Returns
    ///
    /// It returns a `Result` containing a `Stats` struct on success, or a `KaikoUtilError` on failure.
    ///
    /// # Errors
    ///
    /// If there is an error retrieving the metadata statistics from the Kaiko API, a `KaikoUtilError` is returned.
    /// The error message contains the specific error encountered.
    ///
    pub async fn get_stats(&self) -> Result<Stats, KaikoUtilError> {
        self.dbg_print("[get_instruments]: Download metadata statistics from Kaiko.");

        match self.client.get_stats().await {
            Ok(stats) => Ok(stats),

            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving metadata statistics: {}",
                e
            ))),
        }
    }

    /// Retrieves instrument metadata from the Kaiko API.
    ///
    /// This method downloads instrument metadata from the Kaiko API using the `KaikoClient` instance.
    ///
    /// # Returns
    ///
    /// It returns a `Result` containing a `Vec` of `MetaInstrument` structs on success,
    /// or a `KaikoUtilError` on failure.
    ///
    /// # Errors
    ///
    /// If there is an error retrieving the instruments from the Kaiko API, a `KaikoUtilError` is returned.
    /// The error message contains the specific error encountered.
    ///
    pub async fn get_instruments(&self) -> Result<Vec<MetaInstrument>, KaikoUtilError> {
        self.dbg_print("[get_instruments]: Download instrument metadata from Kaiko.");

        match self.client.get_instruments().await {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(KaikoUtilError::new(&format!(
                "Error retrieving instruments: {}",
                e
            ))),
        }
    }
}
