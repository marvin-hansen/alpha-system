use crate::init::InitManager;
use common_errors::prelude::InitError;
use common_metadata::prelude::MetaDataSet;

mod fields;
mod init;
mod utils;

pub async fn download_meta_data(dbg: bool) -> Result<MetaDataSet, InitError> {
    let im = InitManager::new(dbg);
    match im.init().await {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    }
}
