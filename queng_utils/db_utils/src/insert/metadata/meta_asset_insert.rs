use crate::types::{Asset, AssetMetadata};

pub fn generate_asset_insert(asset: &Asset) -> String {
    let table_name = "metadata.assets";
    let code = &asset.code;
    // ClickHouse needs quotes to be escaped
    // https://github.com/ClickHouse/ClickHouse/issues/191
    let name = &asset.name.replace("\'", "\\'");
    let asset_class = &asset.asset_class;
    let asset_figi = extract_asset_figi(&asset.metadata);

    format!(
        r"
        INSERT INTO {table_name} (*)
        VALUES (
        '{code}',
        '{name}',
        '{asset_class}',
        '{asset_figi}'
        )
    "
    )
}

fn extract_asset_figi(metadata: &Option<AssetMetadata>) -> String {
    let empty_string = "".to_string();
    let asset_figi = match metadata {
        Some(metadata) => match &metadata.asset_figi {
            Some(figi) => figi.to_owned(),
            None => empty_string,
        },
        None => empty_string,
    };

    asset_figi
}
