use crate::db::metadata::{Metadata, DB_NAME};
use common::prelude::{Asset, AssetMetadata};

impl Metadata {
    pub(crate) fn generate_asset_insert(&self, asset: &Asset) -> String {
        let table_name = format!("{DB_NAME}.assets");
        let code = asset.code.clone();
        // ClickHouse needs quotes to be escaped
        // https://github.com/ClickHouse/ClickHouse/issues/191
        let name = asset.name.replace('\'', "\\'");
        let asset_class = &asset.asset_class;
        let asset_figi = self.extract_asset_figi(&asset.metadata);

        format!(
            r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
        '{asset_class}',
        '{asset_figi}');"
        )
    }

    fn extract_asset_figi(&self, metadata: &Option<AssetMetadata>) -> String {
        let empty_string = "".to_string();

        match metadata {
            Some(metadata) => match &metadata.asset_figi {
                Some(figi) => figi.to_owned(),
                None => empty_string,
            },
            None => empty_string,
        }
    }
}
