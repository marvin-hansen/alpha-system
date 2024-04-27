use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

mod assets_import;
mod exchanges_import;
mod instruments_import;

impl ClickhouseUtil {
    pub async fn import_all_data(
        &self,
        assets_data_path: &str,
        exchanges_data_path: &str,
        instruments_data_path: &str,
    ) -> Result<(), ClickHouseUtilError> {
        //
        self.import_asset_data(assets_data_path)
            .await
            .expect("Failed to import assets data");

        self.import_exchanges_data(exchanges_data_path)
            .await
            .expect("Failed to import exchanges data");

        self.import_instruments_data(instruments_data_path)
            .await
            .expect("Failed to import instruments data");

        Ok(())
    }
}
