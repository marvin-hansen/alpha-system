use crate::utils::DownloadUtils;
use common_errors::prelude::DownloadError;
use common_metadata::prelude::{
    MetaAsset, MetaAssetRoot, MetaExchange, MetaExchangesRoot, MetaInstrument, MetaInstrumentsRoot,
};

impl DownloadUtils {
    pub(crate) async fn download_assets(&self) -> Result<Vec<MetaAsset>, DownloadError> {
        // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
        match self.download(&self.url_assets).await {
            Ok(bytes) => {
                let assets: MetaAssetRoot =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse assets");

                Ok(assets.data)
            }
            Err(e) => Err(DownloadError::from(format!(
                "Error downloading assets {}",
                e
            ))),
        }
    }

    pub(crate) async fn download_exchanges(&self) -> Result<Vec<MetaExchange>, DownloadError> {
        // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
        match self.download(&self.url_exchanges).await {
            Ok(bytes) => {
                let exchanges: MetaExchangesRoot =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");
                Ok(exchanges.data)
            }
            Err(e) => Err(DownloadError::from(format!(
                "Error downloading exchanges {}",
                e
            ))),
        }
    }

    pub(crate) async fn download_instruments(&self) -> Result<Vec<MetaInstrument>, DownloadError> {
        // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
        match self.download(&self.url_instruments).await {
            Ok(bytes) => {
                let instruments: MetaInstrumentsRoot =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");

                Ok(instruments.data)
            }
            Err(e) => Err(DownloadError::from(format!(
                "Error downloading instruments {}",
                e
            ))),
        }
    }
}
